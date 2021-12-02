#[macro_use]
pub extern crate lazy_static;

pub mod config;
mod input;
mod layout;

use crate::input::bindings::BindingRegistration;
use crate::input::floating::FloatingKeyPressHandler;
use crate::input::input::WmInputHandler;

use crate::layout::floating::FloatingWmLayout;
use crate::layout::layout::WmState;
use std::process::exit;
use x11rb::{
    connection::Connection,
    protocol::{xproto::*, ErrorKind, Event},
    rust_connection::ReplyError,
};

fn main() {
    println!("Starting sfwm (Simple Fucking Window Manager)");

    let (connection, screen_num) = x11rb::connect(None).unwrap();
    let screen = &connection.setup().roots[screen_num];

    let connection = &connection;

    become_wm(connection, screen).unwrap();

    let keybind_registration = BindingRegistration::new();
    let input_handler = &mut WmInputHandler::new(
        connection,
        screen.root,
        &FloatingKeyPressHandler {},
        keybind_registration,
    );

    let mut wm_state =
        WmState::new(connection, &FloatingWmLayout {}, input_handler, screen_num).unwrap();

    wm_state.scan_windows().unwrap();

    loop {
        wm_state.refresh();
        connection.flush().unwrap();

        let event = connection.wait_for_event().unwrap();
        let mut event_option = Some(event);

        while let Some(event) = event_option {
            if let Event::ClientMessage(_) = event {
                return;
            }

            wm_state.handle_event(event).unwrap();
            event_option = connection.poll_for_event().unwrap();
        }
    }
}

fn become_wm<C: Connection>(connection: &C, screen: &Screen) -> Result<(), ReplyError> {
    let change = ChangeWindowAttributesAux::default().event_mask(
        EventMask::SUBSTRUCTURE_REDIRECT
            | EventMask::SUBSTRUCTURE_NOTIFY
            | EventMask::BUTTON_PRESS
            | EventMask::BUTTON_RELEASE
            | EventMask::KEY_PRESS
            | EventMask::KEY_RELEASE,
    );

    let response = connection
        .change_window_attributes(screen.root, &change)?
        .check();

    if let Err(ReplyError::X11Error(ref error)) = response {
        if error.error_kind == ErrorKind::Access {
            eprintln!("Failed to start! Is another WM already running?");
            exit(1);
        }
    }

    return response;
}

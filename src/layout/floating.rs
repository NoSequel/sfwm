use crate::layout::layout::{WindowState, WmLayout, WmState};
use std::cmp::Reverse;
use x11rb::{
    connection::Connection, protocol::xproto::*, rust_connection::ReplyOrIdError,
    COPY_DEPTH_FROM_PARENT, CURRENT_TIME,
};

pub struct FloatingWmLayout;

impl<T: Connection> WmLayout<T> for FloatingWmLayout {
    fn motion_notify(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::MotionNotifyEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if let Some(drag_state) = state.drag_window {
            let x: i32 = (drag_state.x + event.root_x).into();
            let y: i32 = (drag_state.y + event.root_y).into();

            state.connection.configure_window(
                drag_state.window.window,
                &ConfigureWindowAux::new().x(x).y(y),
            )?;
        }

        Ok(())
    }

    fn enter(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::EnterNotifyEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if let Some(window_state) = state.find_window(event.event) {
            state.connection.set_input_focus(
                InputFocus::PARENT,
                window_state.window,
                CURRENT_TIME,
            )?;

            state.connection.configure_window(
                window_state.frame_window,
                &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
            )?;
        }

        Ok(())
    }

    fn leave(&self, state: &mut WmState<T>, event: LeaveNotifyEvent) -> Result<(), ReplyOrIdError> {
        state
            .connection
            .set_input_focus(InputFocus::PARENT, 0 as u32, CURRENT_TIME)?;

        Ok(())
    }

    fn expose(&self, state: &mut WmState<T>, event: x11rb::protocol::xproto::ExposeEvent) {
        state.pending_expose.insert(event.window);
    }

    fn map_request(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::MapRequestEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        self.manage_window(
            state,
            event.window,
            &state.connection.get_geometry(event.window)?.reply()?,
        )?;

        Ok(())
    }

    fn configure_request(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::ConfigureRequestEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if let Some(_) = state.find_window(event.window) {
            unimplemented!();
        }

        let aux = ConfigureWindowAux::from_configure_request(&event)
            .sibling(None)
            .stack_mode(None);

        state.connection.configure_window(event.window, &aux)?;

        Ok(())
    }

    fn unmap_notify(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::UnmapNotifyEvent,
    ) {
        let connection = state.connection;
        let root = connection.setup().roots[state.screen_num].root;

        state.windows.retain(|window_state| {
            if window_state.window != event.window {
                return true;
            }

            connection
                .change_save_set(SetMode::DELETE, window_state.window)
                .unwrap();
            connection
                .reparent_window(window_state.window, root, window_state.x, window_state.y)
                .unwrap();

            connection
                .destroy_window(window_state.frame_window)
                .unwrap();

            return false;
        });
    }

    fn manage_window(
        &self,
        state: &mut WmState<T>,
        window: x11rb::protocol::xproto::Window,
        geometry: &x11rb::protocol::xproto::GetGeometryReply,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        println!("Handling {}", window);

        let connection = state.connection;
        let screen = &connection.setup().roots[state.screen_num];

        let frame_window = state.connection.generate_id()?;
        let window_aux = CreateWindowAux::new().event_mask(
            EventMask::EXPOSURE
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::KEY_PRESS
                | EventMask::KEY_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::ENTER_WINDOW
                | EventMask::LEAVE_WINDOW,
        );

        connection.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_window,
            screen.root,
            geometry.x,
            geometry.y,
            geometry.width,
            geometry.height,
            1,
            WindowClass::INPUT_OUTPUT,
            0,
            &window_aux,
        )?;

        connection.grab_server()?;
        connection.change_save_set(SetMode::INSERT, window)?;

        let cookie = connection.reparent_window(window, frame_window, 0, 0)?;

        connection.map_window(window)?;
        connection.map_window(frame_window)?;

        connection.ungrab_server()?;

        state
            .windows
            .push(WindowState::new(window, frame_window, geometry));

        state
            .sequences_to_ignore
            .push(Reverse(cookie.sequence_number() as u16));

        Ok(())
    }
}

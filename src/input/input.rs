use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::ReplyOrIdError;

struct WmInputHandler<'a, C: Connection> {
    connection: &'a mut C
}

impl<'a, C: Connection> WmInputHandler<'a, C> {
    pub fn new(connection: &'a mut C) -> Self {
        Self {
            connection
        }
    }

    pub fn grab_mouse_input(&self) -> Result<(), ReplyOrIdError> {
        let modifiers = &[0, u16::from(ModMask::M2)];
        let grab_mode = GrabMode::ASYNC;
        let mask = EventMask::BUTTON_PRESS
                        | EventMask::BUTTON_RELEASE
                        | EventMask::BUTTON_MOTION;

        for modifier in modifiers.iter() {
            
        }

        Ok(())
    }
}

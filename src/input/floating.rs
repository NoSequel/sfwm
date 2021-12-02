use crate::input::input::KeyPressHandler;
use x11rb::connection::Connection;

pub struct FloatingKeyPressHandler;

impl<T: Connection> KeyPressHandler<T> for FloatingKeyPressHandler {
    fn button_press(
        &self,
        input_handler: &mut super::input::WmInputHandler<T>,
        event: x11rb::protocol::xproto::ButtonPressEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        Ok(())
    }

    fn button_release(
        &self,
        input_handler: &mut super::input::WmInputHandler<T>,
        event: x11rb::protocol::xproto::ButtonReleaseEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        Ok(())
    }

    fn key_press(
        &self,
        input_handler: &mut super::input::WmInputHandler<T>,
        event: x11rb::protocol::xproto::KeyPressEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        Ok(())
    }

    fn key_release(
        &self,
        input_handler: &mut super::input::WmInputHandler<T>,
        event: x11rb::protocol::xproto::KeyReleaseEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        Ok(())
    }
}

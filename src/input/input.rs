use crate::input::bindings::BindingRegistration;
use crate::input::buffer::KeyBuffer;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::ReplyOrIdError;

pub struct WmInputHandler<'a, C: Connection> {
    pub connection: &'a C,
    pub root: Window,
    pub binding_registration: BindingRegistration,
    pub key_buffer: KeyBuffer,
}

impl<'a, C: Connection> WmInputHandler<'a, C> {
    pub fn new(connection: &'a C, root: Window, binding_registration: BindingRegistration) -> Self {
        let handler = Self {
            connection,
            root,
            binding_registration,
            key_buffer: KeyBuffer::new(),
        };

        return handler;
    }
}

pub struct KeyPressHandler<'a, T: Connection> {
    pub connection: &'a T,
    pub input_handler: &'a mut WmInputHandler<'a, T>,
}

impl<'a, T: Connection> KeyPressHandler<'a, T> {
    pub fn handle_event(&mut self, event: &Event) -> Result<(), ReplyOrIdError> {
        match event {
            Event::ButtonPress(event) => self.button_press(event),
            Event::ButtonRelease(event) => self.button_release(event),
            Event::KeyPress(event) => self.key_press(event),
            Event::KeyRelease(event) => self.key_release(event),
            _ => Ok(()),
        }
    }

    pub fn process_key_grab(&mut self) -> Result<(), ReplyOrIdError> {
        for key in &self.input_handler.binding_registration.key_bindings {
            self.connection.grab_key(
                false,
                self.input_handler.root,
                key.key.mask,
                key.key.code,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
            )?;
        }

        Ok(())
    }

    pub fn button_press(&mut self, event: &ButtonPressEvent) -> Result<(), ReplyOrIdError> {
        Ok(())
    }

    pub fn button_release(&mut self, event: &ButtonReleaseEvent) -> Result<(), ReplyOrIdError> {
        Ok(())
    }

    pub fn key_press(&mut self, event: &KeyPressEvent) -> Result<(), ReplyOrIdError> {
        if let Some(code) = self
            .input_handler
            .binding_registration
            .key_bindings
            .iter()
            .find(|key| key.key.code == event.detail)
        {
            (code.action);
        }

        Ok(())
    }

    pub fn key_release(&mut self, event: &KeyReleaseEvent) -> Result<(), ReplyOrIdError> {
        self.input_handler
            .key_buffer
            .remove_from_buffer(event.detail as usize);
        Ok(())
    }
}

use crate::input::bindings::BindingRegistration;
use crate::input::buffer::KeyBuffer;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::ReplyOrIdError;

pub struct WmInputHandler<'a, C: Connection> {
    pub connection: &'a C,
    pub root: Window,
    pub binding_registration: BindingRegistration<'a>,
    pub key_buffer: KeyBuffer,
}

impl<'a, C: Connection> WmInputHandler<'a, C> {
    pub fn new(
        connection: &'a C,
        root: Window,
        binding_registration: BindingRegistration<'a>,
    ) -> Self {
        let handler = Self {
            connection,
            root,
            binding_registration,
            key_buffer: KeyBuffer::new(),
        };

        handler.grab_key_input();

        return handler;
    }

    pub fn grab_key_input(&self) -> Result<(), ReplyOrIdError> {
        let modifiers = &[0, u16::from(ModMask::M4)];

        for modifier in modifiers {
            for key in &self.binding_registration.key_bindings {
                self.connection.grab_key(
                    false,
                    self.root,
                    key.key.mask | modifier,
                    key.key.code,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                )?;
            }
        }

        Ok(())
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

    pub fn button_press(&mut self, event: &ButtonPressEvent) -> Result<(), ReplyOrIdError> {
        Ok(())
    }

    pub fn button_release(&mut self, event: &ButtonReleaseEvent) -> Result<(), ReplyOrIdError> {
        Ok(())
    }

    pub fn key_press(&mut self, event: &KeyPressEvent) -> Result<(), ReplyOrIdError> {
        let registration = &self.input_handler.binding_registration;

        self.input_handler
            .key_buffer
            .add_to_buffer(event.event as usize);

        for bind in registration.key_bindings.iter() {
            if self.input_handler.key_buffer.is_pressed(bind.key) {
                (bind.action);
            }
        }

        Ok(())
    }

    pub fn key_release(&mut self, event: &KeyReleaseEvent) -> Result<(), ReplyOrIdError> {
        self.input_handler
            .key_buffer
            .remove_from_buffer(event.event as usize);
        Ok(())
    }
}

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::ReplyOrIdError;

use std::collections::HashSet;

use crate::input::bindings::BindingRegistration;
use crate::WmState;

pub struct WmInputHandler<'a, C: Connection> {
    pub connection: &'a mut C,
    pub key_press_handler: &'a dyn KeyPressHandler<C>,
    pub root: Window,
    pub binding_registration: BindingRegistration<'a>,
    pub pressed_masks: HashSet<u16>,
}

impl<'a, C: Connection> WmInputHandler<'a, C> {
    pub fn new(
        connection: &'a mut C,
        root: Window,
        key_press_handler: &'a dyn KeyPressHandler<C>,
        binding_registration: BindingRegistration<'a>,
    ) -> Self {
        let handler = Self {
            connection,
            key_press_handler,
            root,
            binding_registration,
            pressed_masks: HashSet::default(),
        };

        handler.grab_key_input();

        return handler;
    }

    pub fn grab_key_input(&self) -> Result<(), ReplyOrIdError> {
        let modifiers = &[0, u16::from(ModMask::M2)];

        for modifier in modifiers {
            for key in self.binding_registration.key_bindings {
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

pub trait KeyPressHandler<T: Connection> {
    fn button_press(
        &self,
        state: &mut WmState<T>,
        input_handler: &mut WmInputHandler<T>,
        event: ButtonPressEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn button_release(
        &self,
        state: &mut WmState<T>,
        input_handler: &mut WmInputHandler<T>,
        event: ButtonReleaseEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn key_press(
        &self,
        state: &mut WmState<T>,
        input_handler: &mut WmInputHandler<T>,
        event: KeyPressEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn key_release(
        &self,
        state: &mut WmState<T>,
        input_handler: &mut WmInputHandler<T>,
        event: KeyReleaseEvent,
    ) -> Result<(), ReplyOrIdError>;
}

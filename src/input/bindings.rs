use crate::config::Callback;
use x11rb::protocol::xproto::*;

#[derive(Clone)]
pub struct BindingRegistration<'a> {
    pub mouse_bindings: Vec<MouseBinding<'a>>,
    pub key_bindings: Vec<(KeyCode, &'a Callback)>,
}

impl<'a> BindingRegistration<'a> {
    pub fn new() -> Self {
        let registration = Self {
            mouse_bindings: vec![],
            key_bindings: crate::config::KEY_BINDINGS
                .iter()
                .map(|(pattern, action)| {
                    action();
                    (
                        crate::input::keymap::parse_key(pattern.to_string()).unwrap(),
                        action,
                    )
                })
                .collect(),
        };

        return registration;
    }
}

#[derive(Clone, Copy)]
pub struct KeyCode {
    pub mask: u16,
    pub code: u8,
}

#[derive(Clone, Copy)]
pub struct MouseBinding<'a> {
    pub button: Button,
    pub action: &'a Callback,
}

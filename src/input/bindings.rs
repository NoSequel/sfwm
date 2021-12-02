use x11rb::protocol::xproto::*;
use std::collections::HashMap;

pub struct BindingRegistration<'a> {
    pub mouse_bindings: Vec<MouseBinding<'a>>,
    pub key_bindings: Vec<KeyBinding<'a>>,
    pub code_map: HashMap<u8, Vec<String>>
}

impl<'a> BindingRegistration<'a> {
    pub fn new() -> Self {
        Self {
            mouse_bindings: vec![],
            key_bindings: vec![],
            code_map: xmodmap_pke::xmodmap_pke().unwrap()
        }
    }

    pub fn to_key(&self, character: String) -> u8 {
        for (key, options) in self.code_map {
            if let Some(_) = options.iter().find(|option| option == &&character) {
                return key;
            }
        }

        return 0
    }
}

pub struct MouseBinding<'a> {
    pub button: Button,
    pub action: &'a dyn FnMut()
}

impl <'a> MouseBinding<'a> {
    pub fn new(button: Button, action: &'a dyn FnMut()) -> Self {
        Self {
            button,
            action
        }
    }
}

pub struct KeyBinding<'a> {
    pub key: Keycode,
    pub action: &'a dyn FnMut()
}

impl<'a> KeyBinding<'a> {
    pub fn new(key: Keycode, action: &'a dyn FnMut()) -> Self {
        Self {
            key,
            action
        }
    }
}

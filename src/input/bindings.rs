use std::collections::HashMap;
use x11rb::protocol::xproto::*;

pub struct BindingRegistration<'a> {
    pub mouse_bindings: Vec<MouseBinding<'a>>,
    pub key_bindings: Vec<KeyBinding<'a>>,
    pub code_map: HashMap<u8, Vec<String>>,
}

impl<'a> BindingRegistration<'a> {
    pub fn new() -> Self {
        let registration = Self {
            mouse_bindings: vec![],
            key_bindings: vec![],
            code_map: xmodmap_pke::xmodmap_pke().unwrap(),
        };

        registration.key_bindings = registration.read_keybinds();

        return registration;
    }

    pub fn read_keybinds(&self) -> Vec<KeyBinding<'a>> {
        crate::config::KEY_BINDINGS
            .into_iter()
            .map(|(button, action)| match self.parse_key(button.to_owned()) {
                Some(button) => KeyBinding::new(button, &|| action()),
                None => panic!("Unable to parse button pattern {}", button),
            })
            .collect()
    }

    pub fn parse_key(&self, pattern: String) -> Option<KeyCode> {
        let mut parts = pattern.split('-').collect::<Vec<&str>>();

        match self.to_key(parts.remove(parts.len() - 1).to_owned()) {
            Some(code) => {
                let mask = parts
                    .iter()
                    .map(|&option| match option {
                        "A" => u16::from(ModMask::M1),
                        "M" | "Mod" => crate::config::MOD_KEY,
                        "S" | "Shift" => u16::from(ModMask::SHIFT),
                        "C" | "Control" | "Ctrl" => u16::from(ModMask::CONTROL),
                    })
                    .fold(0, |acc, v| acc | v);

                Some(KeyCode {
                    mask: mask as u16,
                    code,
                })
            }
            None => None,
        }
    }

    pub fn to_key(&self, character: String) -> Option<u8> {
        for (key, options) in self.code_map {
            if let Some(_) = options.iter().find(|option| option == &&character) {
                return Some(key);
            }
        }

        return None;
    }
}

pub struct KeyCode {
    pub mask: u16,
    pub code: u8,
}

pub struct MouseBinding<'a> {
    pub button: Button,
    pub action: &'a dyn FnMut(),
}

impl<'a> MouseBinding<'a> {
    pub fn new(button: Button, action: &'a dyn FnMut()) -> Self {
        Self { button, action }
    }
}

pub struct KeyBinding<'a> {
    pub key: KeyCode,
    pub action: &'a dyn FnMut(),
}

impl<'a> KeyBinding<'a> {
    pub fn new(key: KeyCode, action: &'a dyn FnMut()) -> Self {
        Self { key, action }
    }
}

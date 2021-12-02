use std::collections::HashMap;
use x11rb::protocol::xproto::*;

pub struct BindingRegistration {
    pub mouse_bindings: Vec<MouseBinding>,
    pub key_bindings: Vec<KeyBinding>,
    pub code_map: HashMap<u8, Vec<String>>,
}

impl BindingRegistration {
    pub fn new() -> Self {
        let mut registration = Self {
            mouse_bindings: vec![],
            key_bindings: vec![],
            code_map: xmodmap_pke::xmodmap_pke().unwrap(),
        };

        registration.key_bindings = registration.read_keybinds();

        for keybind in &registration.key_bindings {
            (keybind.action);
        }

        return registration;
    }

    pub fn read_keybinds(&self) -> Vec<KeyBinding> {
        for (_, action) in crate::config::KEY_BINDINGS.iter() {
            action();
        }

        crate::config::KEY_BINDINGS
            .iter()
            .map(
                |(button, action)| match self.parse_key(button.to_owned()) {
                    Some(button) => KeyBinding {
                        key: button,
                        action: *action,
                    },
                    None => panic!("Unable to parse button pattern {}", button),
                },
            )
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
                        "M" | "Mod" => u16::from(ModMask::M4),
                        "S" | "Shift" => u16::from(ModMask::SHIFT),
                        "C" | "Control" | "Ctrl" => u16::from(ModMask::CONTROL),
                        _ => 0,
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
        for (key, options) in &self.code_map {
            if let Some(_) = options.iter().find(|option| option == &&character) {
                return Some(*key);
            }
        }

        return None;
    }
}

#[derive(Clone, Copy)]
pub struct KeyCode {
    pub mask: u16,
    pub code: u8,
}

pub struct MouseBinding {
    pub button: Button,
    pub action: fn(),
}

pub struct KeyBinding {
    pub key: KeyCode,
    pub action: fn(),
}

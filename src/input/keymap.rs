use crate::input::bindings::*;
use x11rb::protocol::xproto::*;

pub fn parse_key(pattern: String) -> Option<KeyCode> {
    let mut parts = pattern.split('-').collect::<Vec<&str>>();

    match to_key(parts.remove(parts.len() - 1).to_owned()) {
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

pub fn to_key(character: String) -> Option<u8> {
    for (key, options) in crate::config::KEY_MAP.iter() {
        if let Some(_) = options.iter().find(|option| option == &&character) {
            return Some(*key);
        }
    }

    return None;
}

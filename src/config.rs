use x11rb::protocol::xproto::*;

pub const MOD_KEY: u16 = u16::from(ModMask::M4);

pub const KEY_BINDINGS: Vec<(&str, &dyn FnMut())> = vec![("M-d", &|| println!("hey"))];

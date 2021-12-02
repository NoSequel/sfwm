use std::process::Command;
use x11rb::protocol::xproto::*;

pub fn spawn_command(command: &str) {
    Command::new(command).spawn();
}

pub const MOD_KEY: u16 = u16::from(ModMask::M4);

pub const KEY_BINDINGS: Vec<(&str, &dyn FnMut())> = vec![
    ("M-S-Return", &|| spawn_command("st")),
    ("M-d", &|| spawn_command("dmenu_run")),
];

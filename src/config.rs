use std::collections::HashMap;
use std::process::Command;

pub type Callback = &'static (dyn Fn() + Sync);

pub fn spawn_command(command: &str) {
    Command::new(command).spawn().unwrap();
}

lazy_static! {
    pub static ref KEY_MAP: HashMap<u8, Vec<String>> = xmodmap_pke::xmodmap_pke().unwrap();
    pub static ref KEY_BINDINGS: Vec<(String, Callback)> = vec![
        (String::from("M-Return"), &|| spawn_command("st")),
        (String::from("M-d"), &|| spawn_command("dmenu_run")),
        (String::from("M-b"), &|| println!("hey how are u"))
    ];
}

use std::process::Command;

type Callback = fn();

pub fn spawn_command(command: &str) {
    Command::new(command).spawn().unwrap();
}

lazy_static! {
    pub static ref KEY_BINDINGS: Vec<(String, Callback)> = vec![
        (String::from("M-Return"), || spawn_command("st")),
        (String::from("M-d"), || spawn_command("dmenu_run")),
        (String::from("M-b"), || println!("hey how are u"))
    ];
}

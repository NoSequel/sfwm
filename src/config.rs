use std::process::Command;

pub fn spawn_command(command: &str) {
    Command::new(command).spawn();
}

lazy_static! {
    pub static ref KEY_BINDINGS: Vec<(String, Box<dyn FnMut() + Sync>)> = vec![
        (String::from("M-S-Return"), Box::new(|| spawn_command("st"))),
        (String::from("M-d"), Box::new(|| spawn_command("dmenu_run"))),
    ];
}

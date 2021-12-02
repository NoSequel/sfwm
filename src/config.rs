use std::process::Command;

pub fn spawn_command(command: &str) {
    Command::new(command).spawn().unwrap();
}

lazy_static! {
    pub static ref KEY_BINDINGS: Vec<(String, Box<dyn FnMut() + Sync>)> = vec![
        (String::from("M-Return"), Box::new(|| spawn_command("st"))),
        (String::from("M-d"), Box::new(|| spawn_command("dmenu_run"))),
    ];
}

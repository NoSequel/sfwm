mod layout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, screen_num) = x11rb::connect(None).unwrap();

    println!("Hello, world!");
}

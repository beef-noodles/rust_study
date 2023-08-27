extern crate rpassword;
use std::io::{self, Write};
fn main() {
    println!("Hello, world!");

    print!("Username: ");
    io::stdout().flush().unwrap();

    let mut username_input = String::new();

    io::stdin()
        .read_line(&mut username_input)
        .expect("Wrong username");

    let username = username_input.trim();

    print!("Password: ");
    io::stdout().flush().unwrap();
    let password_input = rpassword::read_password().unwrap();

    let password = password_input.trim();

    println!(
        "username: {}\npassword: {}\norigin password: {}:",
        username, password, password_input
    )
}

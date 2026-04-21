use std::io::{stdin};

fn main() {
    let mut name = String::new();
    println!("What is your name?");
    match stdin().read_line(&mut name) {
        Ok(_) => println!("Hello {}, welcome", name.trim()),
        Err(val) => println!("Error is {val}"),
    }
}

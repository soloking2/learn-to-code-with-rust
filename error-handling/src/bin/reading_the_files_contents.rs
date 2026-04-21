use std::fs::File;
use std::io::{Read, stdin};
use std::process::exit;

fn main() {
    println!("please enter the name of te file you'd like to read:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input. The error was {error}");
        exit(1)
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("An error has occurred. Theerror was {error}");
            exit(1);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprintln!("Something went wrong reading file content. The error was {error}");
        exit(1)
    }


    println!("{file_contents}");
}
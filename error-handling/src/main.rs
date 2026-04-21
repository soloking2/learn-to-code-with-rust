use std::fs;
use std::io::{self, stdin};
use std::process::exit;

fn main() {
    match write_to_file() {
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            exit(1)
        }
    }
}

fn write_to_file() -> Result<String, io::Error> {
    let input = stdin();
    println!("What file would you like to write to?");
    let mut file_name = String::new();
    input.read_line(&mut file_name)?;

    println!("What would you like to write to the file?");
    let mut file_contents = String::new();
    input.read_line(&mut file_contents)?;

    // This uses to file system to either create or open existing file using the given name, and write
    // contents into it
    fs::write(file_name.trim(), file_contents)?;

    Ok(file_name)
}

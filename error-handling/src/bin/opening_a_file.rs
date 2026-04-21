use std::fs::File;
use std::process;

fn main() {
    let file = match File::open("story.txt") {
        Ok(f) => f,
        Err(error) => {
            eprintln!("Something went wrong, {error:?}");
            process::exit(1);
        }
    };
    println!("{file:#?}");
}

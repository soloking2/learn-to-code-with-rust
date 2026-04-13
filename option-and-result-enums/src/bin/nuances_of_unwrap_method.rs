fn main() {
let result = operation(true);

#[allow(unused_variables)]
let content = match result {
    Ok(message) => message,
    Err(error) => error
};

println!("{}", result.unwrap())
}

fn operation(success: bool) -> Result<&'static str, &'static str> {
    if success {
        Ok("Success")
    } else {
        Err("Error")
    }
}
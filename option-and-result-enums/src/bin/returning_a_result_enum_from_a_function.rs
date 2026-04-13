fn main() {
    let divide_10 = divide(10.00, 2.00);

    match divide_10 {
        Ok(value) => println!("The result is: {}", value),
        Err(s) => println!("Error: {}", s)
    }
}

fn divide(numerator: f64, denomindator: f64) -> Result<f64, String> {
    if denomindator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denomindator)
    }
}

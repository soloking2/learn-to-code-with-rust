fn main() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name;
    println!("{full_name}");

    let mut fullname = String::from("Bright");
    fullname.push(' ');
    fullname.push_str(&last_name);

    // Invalid
    println!("{last_name}");
}

fn main() {
    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");


    let arr = [false, true];
    let one = arr[0];
    println!("{one} is part of array {arr:?}");

    let langs = [String::from("Igbo"), String::from("English")];
    let lang = &langs[0];
    println!("{lang} is part of {langs:?}")
}

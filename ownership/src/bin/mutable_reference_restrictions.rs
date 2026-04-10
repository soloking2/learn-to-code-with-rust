fn main() {
    let mut car = String::from("Red");
    let ref1 = &mut car; // A variable can have only one mutable reference at a time
    let ref2 = &mut car;
    println!("{ref2}");
}

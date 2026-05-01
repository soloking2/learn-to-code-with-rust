use std::ops::{Add, Sub};

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sub_two_numbers<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

fn main() {
    let integer_sum = add_two_numbers(1, 2);
    let float_sum = add_two_numbers(1.5, 2.4);
    let integer_diff = sub_two_numbers(5, 3);
    let float_diff = sub_two_numbers(5.0, 3.0);
    println!("{integer_sum} and {float_sum}");
    println!("{integer_diff} and {float_diff}");
}

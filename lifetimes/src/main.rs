/**
 * Define a 'double_the_length' function that accepts a
reference to a vector and returns a usize. The function
should be able to accept a reference to a vector storing
any data type.
 */
fn double_the_length<T>(items: &Vec<T>) -> usize {
    items.len() * 2
}

fn last_two<T>(p: &Vec<T>) -> &[T] {
    &p[p.len()-2..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}

fn fin_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}
fn main() {
    println!("{}", double_the_length(&vec![1,2,3]));
    println!("{}", double_the_length(&vec![1,2,3, 4]));
    println!("{:?}", last_two(&vec![1,2,3]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));
    println!("{:?}", first_five("refrigerator", "Hello"));
    println!("{}", fin_string_that_has_content("programming", "dinning", "gram"));
}

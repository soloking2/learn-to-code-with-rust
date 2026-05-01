fn choose_favorite<'a>(first: &str, second: &'a str) -> &'a str {
    println!("{first}");
    second
}

fn chose_fruit<'a>(a: &str, second: &'a [String]) -> &'a str {
    println!("{a}");
    &second[0]
}

fn main() {}

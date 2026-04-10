#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(13.14));
    println!("{}", identity("hello"));
    println!("{}", identity(String::from("hello")));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));
    println!("{}", check_sum(2025))
}

fn identity<T>(value: T) -> T {
    value
}

fn check_sum<S>(value: S) -> S {
    value
}
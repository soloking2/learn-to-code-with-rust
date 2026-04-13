#[derive(Debug, Copy, Clone)]

enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Oh error")
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value
        }
    }
}

fn main() {
    let some_option = MyOption::Some(50);
    let non_value = MyOption::None;
    println!("{}", some_option.unwrap());
    println!("{}", non_value.unwrap_or(0));
}
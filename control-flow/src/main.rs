fn main() {
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number_refactored("red"));
    factorial(4);
    println!("{}", factorial_recurse(4));
let colors = "green";
    let c = match colors {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    };
    println!("{c}");
    let mut amount = 99;

    while amount > 0 {
        println!("The current value us {amount}");
        amount -= 3;
    }
}

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_refactored(color: &str) -> i32 {
     match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(mut f: i32) {
    let mut answer = 1;
    loop {
        if f == 1 {
            break;
        }
        answer *= f;
        f -= 1;
    }
    println!("{answer}");
}

fn factorial_recurse(f: i32) -> i32 {
    if f == 1 {
        return 1;
    }
    return f * factorial_recurse(f - 1);
}

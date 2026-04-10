#[derive(Debug)]
enum Milk {
    Whole,
    Lowfat(i32),
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Lowfat(2025);

    let Milk::NonDairy { kind } = my_beverage else {
        println!("You do not have the nondairy milk");
        return;
    };

    println!("{kind} milk is available here");
}

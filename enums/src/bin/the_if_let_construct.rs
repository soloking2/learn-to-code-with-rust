enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    if let Milk::Whole = my_beverage {
        println!("You have the Whole milk");
    }

    let my_beverage = Milk::Lowfat(3);

    //If my_beverage is equal to Milk::Lowfat(percent: i32),run the condition
    if let Milk::Lowfat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk.");
    }

    let whole_milk = Milk::Whole;
    if let Milk::Whole = whole_milk {
        println!("You have a new kind of milk");
    }

    let my_beverage = Milk::NonDairy {
        kind: String::from("oat"),
    };

    if let Milk::NonDairy { kind } = my_beverage {
        println!("{kind} milk");
    }
}

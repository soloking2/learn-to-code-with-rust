use std::fmt::{Display, Formatter, Debug, Result};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("This drink has the following data: {}", self.get_data())
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond
}

use Milk::{Whole, Oat, Almond};
struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind: kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("** Coffee **")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces -= 1;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {} ", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: &str) -> Self {
        Self { calories, price, flavor: flavor.to_string(), percentage: 100 }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self { calories: self.calories, price: self.price, flavor: self.flavor.clone(), percentage: self.percentage }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}
fn main() {
    let mut latte = Coffee::new("three_in_one", Whole, 8);

    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new("capuccino".to_string(), Oat, 12);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(100, 5000.05, "Cherry");
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("Is the pepsi the same price as the coke? {}\n", pepsi == coke);
    coke.consume();
    
    println!("{:?}", coke);
}
fn print_in_two_formats<T: Display + Debug>(output: T) {
    println!("{}", output);
    println!("{:?}", output)
}
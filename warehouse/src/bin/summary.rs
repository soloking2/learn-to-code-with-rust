
use fake::{Fake, Faker};
use warehouse::{Item};

/// This file summarizes the warehouse
fn main() {
    let fruits: Item = Faker.fake();
    println!("{:?}", fruits);
}
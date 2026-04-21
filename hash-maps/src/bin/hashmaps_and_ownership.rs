use std::collections::HashMap;

fn main() {
    let mut cofee_pairings: HashMap<&str, &str> = HashMap::new();
    let milk = String::from("Milk");
    let brand = String::from("Cowbell");
    let milo = String::from("Milo");
    cofee_pairings.insert(&milk, &brand);
    cofee_pairings.insert("Chocolate", &milo);

    println!("{:?}", cofee_pairings)
}
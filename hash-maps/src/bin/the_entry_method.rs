use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");

    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Pistachio Milk");

    // Put an entry into the hashmap if the key passed to entry is not in the hashmap
    coffee_pairings.entry("Milk").or_insert("Milo");
    println!("{coffee_pairings:?}");
}

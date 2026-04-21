use std::collections::HashMap;

fn main() {
    let meals = [("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]), ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"])];
    let mut sauce_to_meals: HashMap<&str, Vec<&str>> = HashMap::from(meals);
    sauce_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("{:?}", sauce_to_meals);

     match sauce_to_meals.remove("Mayonnaise") {
        Some(ve) => println!("{:?}", ve),
        None => println!("Nothing found")
    }
    let mustard = sauce_to_meals.get("Mustard")
    .cloned()
    .unwrap_or(vec!["Error"]);

    println!("{:?}", mustard);

    sauce_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
    println!("{:?}", sauce_to_meals);

    let plane = String::from("Boeing");
    let passenger_count = 160;
    let travel_options = HashMap::from([(plane, passenger_count)]);


}

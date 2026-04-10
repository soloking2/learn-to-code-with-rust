fn main() {
    let is_concert = true;
    let is_event = is_concert;

    println!("{is_concert} {is_event}");

    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi} {dinner}");

    let sushi = String::from("Salmon");
    let dinner = sushi;

    let a = eat_meal(dinner);
    println!("{a}");

    let mut trip = start_trip();
    visit_philadephia(&mut trip);
    trip.push_str("and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    show_itinerary(&trip);
    //A help allocate
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}

fn start_trip() -> String {
     String::from("The plan is...")
}

fn visit_philadephia(trip: &mut String) {
    trip.push_str("Philadephia ");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

fn visit_boston(trip: &mut String) {
    trip.push_str(" Boston.");
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}
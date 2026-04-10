#[derive(Debug)]

struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self { origin, destination, price, passengers }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.20;
    }

    fn itinerary(&self) {
        println!("({} -> {})", self.origin, self.destination)
    }
}
fn main() {
    let mut first_flight = Flight::new(String::from("Lagos"), String::from("Abuja"), 90560.05, 20);

    first_flight.change_destination(String::from("Owerri"));
    first_flight.increase_price();
    first_flight.itinerary();

    println!("{first_flight:#?}");

    let second_flight =  Flight{origin: String::from("Lagos"), destination: String::from("Enugu"), ..first_flight};
    second_flight.itinerary();

    println!("{second_flight:#?}");

}
use traits::{lodging, utils::{book_for_one_night, book_two_nights}};
use lodging::{Accommodation, Hotel, Airbnb};



fn main() {
    let mut hotel = Hotel::new("Citiheights");
    hotel.book("Solomon", 4);
    book_for_one_night(&mut hotel, "Emmanuel");
    println!("{:?}", hotel);

    println!("============================");
    let mut airbnb = Airbnb::new("Peter");
    airbnb.book("Solomon", 2);
    book_two_nights(&mut airbnb, "Amanda");
    println!("{}", airbnb.get_description());
    println!("{}", airbnb.summarize());
    println!("{:?}", airbnb);
}

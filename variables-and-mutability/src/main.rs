type Meters = i32;
const TOUCHDOWN_POINTS: i32 = 6;

//Directive to allow unused variables
#[allow(unused_variables)]
fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    println!("This year, my garden has {apples} apples and {oranges} oranges. ");

let mile_race_length: Meters = 1600;
let two_mile_race_length: Meters = 3200;
println!("{mile_race_length} is lower than {two_mile_race_length}");

let season: &str = "rainy";
let mut points_scored: i32 = 28;
let event_time = "06:00";
let event_time = 6;
points_scored = 35;

println!("{season} {points_scored} {event_time} {TOUCHDOWN_POINTS}");
let favorite_beverage = "cowbell milk";


}

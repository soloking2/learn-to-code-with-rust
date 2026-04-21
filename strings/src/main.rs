use std::io::stdin;

fn main() {
    let mut name = String::from("Money");
    make_money(&mut name);
    println!("{}", name);
    println!("{}", trim_and_capitalize("Title"));
    println!("{:?}", elements("Gold!Silver!Platinum"));

    let s = get_identity();
    println!("{}", s);
    
}

fn make_money(name: &mut String) {
 name.push_str("$$$")
}

fn trim_and_capitalize(title: &str) -> String{
    return title.trim().to_uppercase();
}

fn elements(s: &str) -> Vec<&str>{
    s.split("!").collect()
}

fn get_identity() -> String {

    let mut first = String::new();
    let mut last = String::new();
    let input = stdin();
    println!("Enter your first name:");
   input.read_line(&mut first).expect("Failed to collect first name");
   println!("Enter your last name:");
   input.read_line(&mut last).expect("Failed to collect last name");

   format!("{} {}", first.trim(), "ghtjtjj")

}
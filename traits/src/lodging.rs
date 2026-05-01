use std::collections::HashMap;

pub trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}
#[derive(Debug)]
pub struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
   pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), reservations: HashMap::new() }
    }
}

impl Accommodation for Hotel {
   fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}
#[derive(Debug)]
pub struct Airbnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl Airbnb {
   pub fn new(host: &str) -> Self {
        Self { host: host.to_string(), guests: vec![] }
    }

   pub fn summarize(&self) -> String{
        format!("{}: {}", self.host, self.get_description())
    }
}

impl Accommodation for Airbnb {
    fn get_description(&self) -> String {
        format!("Please enoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}
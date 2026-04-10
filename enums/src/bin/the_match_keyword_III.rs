#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot {temperature: u32},
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
    match self {
        LaundryCycle::Cold => {
            println!("Runnning the laundry with cold temperature");
        }
        LaundryCycle::Hot {temperature} => {
            println!("Running the laundry with a temperation of {temperature}");
        }
        LaundryCycle::Delicate(name) => {
            println!("Running the laundry with a delicate cycle for {name}");
        }
    }
}
}

fn main() {
     LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 43 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Medium")).wash_laundry();
}

//using Method for using enum
fn _wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Runnning the laundry with cold temperature");
        }
        LaundryCycle::Hot {temperature} => {
            println!("Running the laundry with a temperation of {temperature}");
        }
        LaundryCycle::Delicate(name) => {
            println!("Running the laundry with a delicate cycle for {name}");
        }
    }
}




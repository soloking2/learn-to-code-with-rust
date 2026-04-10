enum Milk {
    Lowfat(i32),
    HighFat(u32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::HighFat(100) => {
                println!("Very high calorie, 100% is very high!");
            }
            Milk::HighFat(item) => {
                println!("High calorie, {item}% is high!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

fn main() {
    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::HighFat(100).drink();
    Milk::HighFat(59).drink();
    Milk::Whole.drink();
}

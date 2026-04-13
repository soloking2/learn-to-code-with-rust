#[derive(Debug)]
struct Food {
    name: String
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return Option::None;
        }
        if self.reservations < 12 {
            Option::Some(Food {
                name: "Uni Sashimi".to_string()
            })
        } else {
            Option::Some(Food {
                name: "Strip Steak".to_string()
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err("Sorry, we have a mice problem".to_string())
        } else if address.is_empty() {
            Err("No delivery address specified".to_string())
        } else {
            Ok(Food {
                name: "Burger".to_string()
            })
        }
    }
}

fn main() {

    let restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true
    };
    
    println!("{:?}", restaurant.chef_special());
    println!("{:?}", restaurant.deliver_burger("123 Elm Street"));

    let res2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false
    };
    println!("{:?}", res2.chef_special());
    println!("{:?}", res2.deliver_burger(""));
    println!("{}", res2.deliver_burger("123 Elm Street").expect("Name of the food not available").name);

     let a: Result<i32, &str> = Ok(5);
    println!("{}", a);

}

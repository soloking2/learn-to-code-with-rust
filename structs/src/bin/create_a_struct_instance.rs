struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }

fn main() {

    
    let three_in_one = make_coffee(String::from("Three in one"), 900.20,true);
    println!("My {} this morning cost {}. It was {} it was hot", three_in_one.name, three_in_one.price, three_in_one.is_hot);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot
    }
}

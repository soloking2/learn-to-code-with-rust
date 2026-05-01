use std::fmt::{Display, Formatter, Result};

enum AppleType {
    Vegetables,
    Legumes
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::Vegetables => write!(formatter, "Delicious"),
            AppleType::Legumes => write!(formatter, "Medicinal"),

        }
    }
}
struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        //We are writing to the formater using write
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}


fn main() {
    let fruit = Apple {
        kind: AppleType::Vegetables,
        price: 600.00
    };
    println!("{}", fruit);
    
    let beans = Apple {
        kind: AppleType::Legumes,
        price: 3000.00
    };
    println!("{}", beans);
}

trait Taxable {
    const TAX_RATE: f64 = 0.25;
    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

struct Bonus {
    amount: f64,
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.15;
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

fn main() {
    let income  = Income {amount: 500.52};
    println!("Total tax owed: N{:.2}", income.tax_bill());

    let bonus  = Bonus {amount: 400.00};
    println!("Total tax owed: N{:.2}", bonus.tax_bill());
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
    PayStack(String),
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298-2093-4800"));
    let paystack = PaymentMethodType::PayStack(String::from("card"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);
    println!("{:#?}", paystack);
}

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
#[derive(Debug)]
struct Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    let first_card = CardSuit::Hearts;
    let third_card = Card {
        rank: String::from("Deputy"),
        suit: CardSuit::Diamonds
    };
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
    println!("{}", third_card.rank);
}

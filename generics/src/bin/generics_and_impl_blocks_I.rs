#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl TreasureChest<i32> {
    fn calculate_amount(&mut self) {
        self.treasure *=  self.treasure;
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{}", gold_chest.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("     Silver    "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest);

    let mut treasure_amount = TreasureChest {
        captain: String::from("BountyHunter"),
        treasure: 7
    };
    println!("{:?}", treasure_amount.calculate_amount());
    println!("{:?}", treasure_amount);
}

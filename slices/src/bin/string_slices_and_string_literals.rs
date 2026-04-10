fn main() {
    let action_hero = String::from("James bond");
    let last_name = &action_hero[6..];
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    println!("{first_name}"); // Arnold
    println!("{last_name}");
}

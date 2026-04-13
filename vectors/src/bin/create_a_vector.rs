fn main() {
    let pizza_diameter: Vec<i32> = Vec::new();
    println!("{pizza_diameter:?}");

    let mut pastas = vec![8,10,12,34];
    println!("{pastas:?}");
    
    pastas.pop();
    println!("{pastas:?}");
}

/**Mod is collection of related codes */
/*These fields inside the mod are private to the scope at which the mod is written
*If we want to make it public, we use pub prefix
*/
// mod inventory;
// mod orders;

// use inventory::{ MANAGER, FLOOR_SPACE};
// use inventory::products::{Item, ProductCategory};

use fake::{Fake, Faker};
use warehouse::{MANAGER, FLOOR_SPACE, ORDERS_MANAGER, Item, ProductCategory};

fn main() {
    println!("Our managers are {} and {}, and there are {} floor space", MANAGER, ORDERS_MANAGER, FLOOR_SPACE);
    
    let apple: Item = Faker.fake();
    println!("{:#?}", apple);
    
    let random_category: ProductCategory  = Faker.fake();
    println!("{:?}", random_category);
}

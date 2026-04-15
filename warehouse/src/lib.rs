/// This module represent tools for inventory
pub mod inventory;

/// This module represent tools for orders
pub mod orders;

pub use inventory::{ MANAGER, FLOOR_SPACE};
pub use inventory::products::{Item, ProductCategory};
pub use orders::MANAGER as ORDERS_MANAGER;
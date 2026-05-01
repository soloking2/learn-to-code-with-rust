// use super::lodging::{Accommodation}; - using super
use crate::lodging::Accommodation;

/**
 * Entity - It is used for any struct that implements the Accommodation trait
 * &impl Accommodation - Have an immutable reference for entity
 */
pub fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

/**
 * Trait Bound
 */
pub fn book_two_nights<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 2);
}

pub fn mix_and_match<T, U>(first: &mut  T, second: &mut U, guest: &str)
where 
    T: Accommodation,
    U:Accommodation
     {
     first.book(guest, 1);
     second.book(guest, 1);
}
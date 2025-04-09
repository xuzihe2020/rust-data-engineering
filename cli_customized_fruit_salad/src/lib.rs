//! This code defines a function called create_fruit_salad
//! that takes a mutable vector of strings as input and returns
//! a new vector of strings that contains the same elements as the input vector,
//! but in a random order.

use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(all_fruit: &mut Vec<String>) {
    let mut rng = rng();
    all_fruit.shuffle(&mut rng);
}

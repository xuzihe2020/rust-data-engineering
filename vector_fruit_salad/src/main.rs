//! This program creates a fruit salad by scrambling (shuffling) a list of fruit.
//! A vector is a growable array. It can grow or shrink in size and is one of the most
//! useful data structures in Rust. A vector is represented using the Vec<T> type.

use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    let mut fruit: Vec<&str> = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
    ];

    let mut rng: rand::prelude::ThreadRng = rng();
    fruit.shuffle(&mut rng);

    // Print the (shuffled) fruit salad
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
}

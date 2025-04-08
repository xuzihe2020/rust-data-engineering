use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::collections::HashSet;

fn generate_random_fruit() -> &'static str {
    let all_fruit = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
        "Pineapple"
    ];
    let mut rng = rng();
    all_fruit.choose(&mut rng).unwrap()
}

fn main() {
    let mut random_fruit = HashSet::new();
    println!("Generate random fruit");
    for _ in 0..100 {
        random_fruit.insert(generate_random_fruit());
    }
    println!("Number of random fruit are {}", random_fruit.len());
}

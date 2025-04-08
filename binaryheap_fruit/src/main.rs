use rand::prelude::IndexedRandom;
use rand::rng;
use rand::seq::SliceRandom;
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
enum Fruit {
    Pineapple,
    Mango,
    Other(String),
}

/// Define pineapple > mango > other.
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Pineapple, Fruit::Pineapple) => std::cmp::Ordering::Equal,
            (Fruit::Pineapple, _) => std::cmp::Ordering::Greater,
            (Fruit::Mango, Fruit::Pineapple) => std::cmp::Ordering::Less,
            (Fruit::Mango, Fruit::Mango) => std::cmp::Ordering::Equal,
            (Fruit::Mango, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
            (Fruit::Other(_), _) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = rng();
    let all_fruit = vec![
        "Apple",
        "Orange",
        "Pear",
        "Peach",
        "Banana",
        "Fig",
        "Pineapple",
        "Mango",
        "Watermelon",
        "Strewberry",
    ];

    let mut fruit_salad = BinaryHeap::new();
    let mut pineapple_count = 0;
    while pineapple_count < 3 {
        let fruit = all_fruit.choose(&mut rng).unwrap();
        if *fruit == "Pineapple" {
            pineapple_count += 1;
            fruit_salad.push(Fruit::Pineapple);
        } else if *fruit == "Mango" {
            fruit_salad.push(Fruit::Mango);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }
    fruit_salad
}

fn main() {
    let fruit_salad = generate_fruit_salad();
    println!("Random Fruit Salad With 3 Servings of Pineapple:");
    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Pineapple => println!("Pineapple"),
            Fruit::Mango => println!("Mango"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}

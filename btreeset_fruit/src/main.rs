use rand::rng;
use rand::seq::SliceRandom;
use std::collections::BTreeSet;

fn main() {
    let all_fruit = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "pineapple",
    ];

    let amounts = [1, 3, 5, 7, 9];
    for amount in amounts.iter() {
        let mut rng = rng();
        let mut shuffled_fruit = all_fruit.clone();
        shuffled_fruit.shuffle(&mut rng);

        let mut fruit_set = BTreeSet::new();
        for fruit in shuffled_fruit {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }
        println!(
            "When amount is {}: inserted fruit are {:?}",
            amount, fruit_set
        );
    }
}

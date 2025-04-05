//! This code starts with an initial VecDeque,
//! converts it to a Vec for shuffling, and then converts it back to a VecDeque.
//! After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
//! to the back of the deque. Finally, it prints out the final fruit salad.

//! A VecDeque is a double-ended queue, which means that you can push and pop from both ends
//! of the queue.

use rand::rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::from(vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
    ]);

    fruit.push_back("pomegranate");
    fruit.push_back("kiwi");
    fruit.push_back("lemon");

    // Scramble the fruit salad
    let mut rng: rand::prelude::ThreadRng = rng();
    let mut fruit: Vec<&str> = fruit.iter().map(|&s| s).collect();
    fruit.shuffle(&mut rng);

    // Convert back to VecDeque
    let mut fruit: VecDeque<&str> = VecDeque::from(fruit);

    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    println!("Fruit salad: ");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
}

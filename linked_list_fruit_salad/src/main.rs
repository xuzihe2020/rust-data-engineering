//! As with the VecDeque example, this code starts by creating a LinkedList of fruits,
//! converts it to a Vec for shuffling, and then converts it back to a LinkedList.
//! After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
//! Finally, it prints out the final fruit salad.
//!
//! This example shows how to use a LinkedList, but remember that LinkedList
//! has a higher memory overhead and worse cache locality than Vec or VecDeque,
//! so it's typically not the best choice unless you have a specific need for the properties
//! of a linked list. In Rust, it's usually better to use a Vec or VecDeque.
//!
//! A LinkedList is a doubly-linked list, which means that each element in the list
//! has a pointer to the next element and the previous element.
//! A great example of when to use a LinkedList is when you need to insert or remove elements
//! from the middle of the list.

use rand::rng;
use rand::seq::SliceRandom;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("pomegranate");
    fruit.push_back("kiwi");
    fruit.push_back("lemon");

    let mut rng: rand::prelude::ThreadRng = rng();
    let mut fruit: Vec<&str> = fruit.iter().map(|&s| s).collect();
    fruit.shuffle(&mut rng);

    // Convert back to LinkedList
    let mut fruit: LinkedList<&str> = fruit.into_iter().collect();
    fruit.push_back("Pomegranate");
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

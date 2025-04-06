//! This example code counts the frequency of each number in the vector.
//!
use std::collections::HashMap;

fn logic(numbers: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frequency: &mut i32 = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    for (num, frequency) in frequencies {
        result.push((*num, frequency));
    }

    result
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 1, 2, 3, 1];
    let result = logic(&numbers);
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
    println!("The original vector is: {:?}", numbers);
}

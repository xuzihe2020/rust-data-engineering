use hashmap_language::{calculate_weights, init_languages};

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}

//! Usage:  
//!
//! cargo run -- fruit.csv
//! or
//! cargo run -- --fruit "apple, pear"

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "xuzihetony@gmail.com",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruit: Option<String>,
    csvfile: Option<String>,
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}

fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut all_fruit = match opts.csvfile {
        Some(csv) => {
            let fruit = std::fs::read_to_string(csv).expect("Could not read file");
            csv_to_vec(&fruit)
        }
        None => opts
            .fruit
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    create_fruit_salad(&mut all_fruit);
    display_fruit_salad(all_fruit);
}

use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let num_fruits = opts.number;

    let fruit_salad: Vec<String> = create_fruit_salad(num_fruits);

    println!(
        "Created fruilt salad with {} fruits: {:?}",
        num_fruits,
        fruit_salad
    );
}
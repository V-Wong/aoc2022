mod q1;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    question: String,
    input_path: String,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(&args.input_path).expect("Could not open file");

    let solution: i32 = match args.question.as_str() {
        "q1a" => q1::solve_a(&input),
        "q1b" => q1::solve_b(&input),
        _ => panic!("Invalid question"),
    };

    println!("Solution to {}: {}", args.question, solution);
}

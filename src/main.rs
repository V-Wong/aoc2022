mod q1;
mod q2;
mod q3;
mod q4;

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

    let solution: Box<dyn std::fmt::Debug> = match args.question.as_str() {
        "q1a" => Box::new(q1::solve_a(&input)),
        "q1b" => Box::new(q1::solve_b(&input)),
        "q2a" => Box::new(q2::solve_a(&input)),
        "q2b" => Box::new(q2::solve_b(&input)),
        "q3a" => Box::new(q3::solve_a(&input)),
        "q3b" => Box::new(q3::solve_b(&input)),
        "q4a" => Box::new(q4::solve_a(&input)),
        "q4b" => Box::new(q4::solve_b(&input)),
        _ => panic!("Invalid question"),
    };

    println!("Solution to {}: {:?}", args.question, solution);
}

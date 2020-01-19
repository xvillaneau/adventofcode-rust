use std::env;
use std::process;

use advent_of_code::Puzzle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle = Puzzle::from_args(&args).unwrap_or_else(|err| {
        println!("Couldn't load puzzle: {:?}", err);
        process::exit(1);
    });
    puzzle.dispatch();
}


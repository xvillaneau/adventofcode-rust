use std::env;
use std::fs;
use std::io;
use std::process;
use std::time::Instant;

mod aoc_2019;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle = Puzzle::from_args(&args).unwrap_or_else(|err| {
        println!("Couldn't load puzzle: {:?}", err);
        process::exit(1);
    });
    puzzle.dispatch();
}

struct Puzzle {
    year: u16,
    day: u8,
    input: String
}

impl Puzzle {
    fn new(year: u16, day: u8) -> Result<Puzzle, &'static str> {
        match read_puzzle_input(year, day) {
            Ok(input) => Ok(Puzzle { year, day, input }),
            Err(_) => Err("Couldn't load puzzle input"),
        }
    }

    fn from_args(args: &[String]) -> Result<Puzzle, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let year = match args[1].parse() {
            Ok(year) => year,
            Err(_) => return Err("invalid year"),
        };
        let day = match args[2].parse() {
            Ok(day) => day,
            Err(_) => return Err("invalid day"),
        };

        Puzzle::new(year, day)
    }

    fn dispatch(&self) {
        println!("=== Advent of Code {}, day {}", self.year, self.day);
        let now = Instant::now();
        match self.year {
            2019 => aoc_2019::dispatch(self.day, &self.input),
            _ => println!("Year {} not available", self.year),
        };
        println!("    Ran in {} µs", now.elapsed().as_micros());
    }
}

fn read_puzzle_input(year: u16, day: u8) -> io::Result<String> {
    let filename = format!("data/{}/day_{:02}.txt", year, day);
    fs::read_to_string(filename)
}


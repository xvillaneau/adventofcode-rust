use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::time::Instant;

pub mod aoc_2019;

pub struct Puzzle {
    year: u16,
    day: u8,
    input: String
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Result<Puzzle, PuzzleError> {
        let filename = format!("data/{}/day_{:02}.txt", year, day);
        let input = fs::read_to_string(filename)?;
        Ok(Puzzle { year, day, input })
    }

    pub fn from_args(args: &[String]) -> Result<Puzzle, PuzzleError> {
        if args.len() < 3 {
            return Err(PuzzleError::new("not enough arguments"));
        }

        let year = args[1].parse()?;
        let day = args[2].parse()?;

        Puzzle::new(year, day)
    }

    pub fn dispatch(&self) {
        println!("=== Advent of Code {}, day {}", self.year, self.day);
        let now = Instant::now();
        match self.year {
            2019 => aoc_2019::dispatch(self.day, &self.input),
            _ => println!("Year {} not available", self.year),
        };
        println!("    Ran in {} µs", now.elapsed().as_micros());
    }
}

// Error definition

#[derive(Debug)]
pub struct PuzzleError {
    details: String
}

impl PuzzleError {
    pub fn new(msg: &str) -> PuzzleError {
        PuzzleError { details: msg.to_string() }
    }
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PuzzleError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for PuzzleError {
    fn from(err: ParseIntError) -> Self {
        PuzzleError::new(err.description())
    }
}

impl From<io::Error> for PuzzleError {
    fn from(err: io::Error) -> Self {
        PuzzleError::new(err.description())
    }
}

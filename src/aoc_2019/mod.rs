mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_22;

pub fn dispatch(day: u8, puzzle_input: &String) {
    match day {
         1 => day_01::main(&puzzle_input),
         2 => day_02::main(&puzzle_input),
         3 => day_03::main(&puzzle_input),
         4 => day_04::main(&puzzle_input),
        22 => day_22::main(&puzzle_input),
         _ => println!("Day {} not found", day),
    }
}

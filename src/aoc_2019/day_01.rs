
pub fn main(puzzle_input: &String) {
    let mut direct_fuel = 0;
    let mut total_fuel = 0;

    for line in puzzle_input.lines() {
        let mass = line.trim().parse().unwrap_or(0);
        direct_fuel += fuel_cost(mass);
        total_fuel += total_fuel_cost(mass);
    }

    println!("Part 1: {}", direct_fuel);
    println!("Part 2: {}", total_fuel);
}

fn fuel_cost(mass: u32) -> u32 {
    if mass >= 6 {
        (mass / 3) - 2
    } else {
        0
    }
}

fn total_fuel_cost(mass: u32) -> u32 {
    let mut total_fuel = 0;
    let mut mass = fuel_cost(mass);

    while mass > 0 {
        total_fuel += mass;
        mass = fuel_cost(mass);
    };

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_cost() {
        assert_eq!(2, fuel_cost(12));
        assert_eq!(2, fuel_cost(14));
        assert_eq!(654, fuel_cost(1969));
        assert_eq!(33583, fuel_cost(100756));
    }

    #[test]
    fn test_total_fuel_cost() {
        assert_eq!(2, total_fuel_cost(14));
        assert_eq!(966, total_fuel_cost(1969));
        assert_eq!(50346, total_fuel_cost(100756));
    }
}


pub fn main(puzzle_input: &String) {
    if let Some(context) = Context::parse(puzzle_input) {
        println!("Part 1: {}", part_1(&context));
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Context {
    start: u32,
    stop: u32,
    start_digit: u32,
}

impl Context {
    const MAX: u32 = 100_000;

    fn new(start: u32, stop: u32) -> Self {
        let start_digit = start / Context::MAX;
        Context {
            start,
            stop,
            start_digit,
        }
    }

    fn parse(string: &str) -> Option<Self> {
        let mut elems = string.trim().split("-");
        let start = match elems.next().unwrap_or("").parse() {
            Ok(value) => value,
            _ => return None,
        };
        let stop = match elems.next().unwrap_or("").parse() {
            Ok(value) => value,
            _ => return None,
        };
        Some(Context::new(start, stop))
    }
}

#[derive(Debug, PartialEq)]
struct Part1State<'a> {
    context: &'a Context,
    number: u32,
    magnitude: u32,
    has_pair: bool,
}

impl<'a> Part1State<'a> {
    fn init(context: &'a Context) -> Part1State<'a> {
        Part1State {
            context,
            number: 0,
            magnitude: 0,
            has_pair: false,
        }
    }

    fn add_digit(&self, digit: u32) -> Option<Self> {
        if self.magnitude == 0 {
            return Some(Part1State {
                number: digit,
                magnitude: 1,
                ..*self
            });
        };

        let last_digit = self.number / self.magnitude;
        if digit > last_digit {
            return None;
        };

        let magnitude = self.magnitude * 10;
        Some(Part1State {
            number: self.number + digit * magnitude,
            has_pair: self.has_pair || digit == last_digit,
            magnitude,
            ..*self
        })
    }

    fn next_states(&self) -> Vec<Self> {
        let mut states = Vec::new();
        for digit in self.context.start_digit..10 {
            if let Some(state) = self.add_digit(digit) {
                states.push(state);
            }
        }
        states
    }

    fn is_final(&self) -> bool {
        self.magnitude == Context::MAX
    }

    fn is_valid(&self) -> bool {
        self.has_pair && self.number <= self.context.stop && self.number >= self.context.start
    }
}

fn part_1(context: &Context) -> u32 {
    let mut stack = Vec::new();
    let mut count: u32 = 0;

    stack.push(Part1State::init(context));
    while stack.len() > 0 {
        let state = stack.pop().unwrap();
        if !state.is_final() {
            stack.append(&mut state.next_states());
        } else if state.is_valid() {
            count += 1;
        };
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_states_1() {
        let context = Context::new(900_000, 999_999);
        let state = Part1State::init(&context);
        let next_states = state.next_states();
        assert_eq!(1, next_states.len());

        let new_state = &next_states[0];
        assert_eq!(9, new_state.number);
        assert_eq!(1, new_state.magnitude);
        assert_eq!(false, new_state.has_pair);
    }

    #[test]
    fn test_next_states_2() {
        let context = Context::new(800_000, 999_999);
        let state = Part1State::init(&context);
        let expected = vec![
            Part1State {
                context: &context,
                number: 8,
                magnitude: 1,
                has_pair: false,
            },
            Part1State {
                context: &context,
                number: 9,
                magnitude: 1,
                has_pair: false,
            },
        ];
        assert_eq!(expected, state.next_states());

        let expected_2 = vec![
            Part1State {
                context: &context,
                number: 89,
                magnitude: 10,
                has_pair: false,
            },
            Part1State {
                context: &context,
                number: 99,
                magnitude: 10,
                has_pair: true,
            },
        ];
        assert_eq!(expected_2, expected[1].next_states());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1, part_1(&Context::new(111_111, 111_111)));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Some(Context::new(123456, 654321)),
            Context::parse("123456-654321")
        );
    }
}

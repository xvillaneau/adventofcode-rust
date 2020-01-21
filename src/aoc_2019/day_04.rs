pub fn main(puzzle_input: &String) {
    if let Some(context) = Context::parse(puzzle_input) {
        let (part_1, part_2) = count_valid(&context);
        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);
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

    fn contains(&self, number: u32) -> bool {
        number >= self.start && number <= self.stop
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
struct State<'a> {
    context: &'a Context,
    number: u32,
    magnitude: u32,
    has_pair: bool,
    has_true_pair: bool,
    streak: u8,
}

impl<'a> State<'a> {
    fn init(context: &'a Context) -> State<'a> {
        State {
            context,
            number: 0,
            magnitude: 0,
            has_pair: false,
            has_true_pair: false,
            streak: 0,
        }
    }

    fn next_states(&self) -> Vec<Self> {
        let (last_digit, magnitude) = if self.magnitude > 0 {
            (self.number / self.magnitude, self.magnitude * 10)
        } else {
            (9, 1)
        };

        let mut states = Vec::new();

        for digit in self.context.start_digit..last_digit + 1 {
            let (has_pair, has_true_pair, streak) = if self.magnitude > 0 {
                (
                    self.has_pair || digit == last_digit,
                    self.has_true_pair || (digit != last_digit && self.streak == 2),
                    if digit == last_digit {
                        self.streak + 1
                    } else {
                        1
                    },
                )
            } else {
                (false, false, 1)
            };

            states.push(State {
                number: self.number + digit * magnitude,
                magnitude,
                has_pair,
                has_true_pair,
                streak,
                ..*self
            });
        }

        states
    }

    fn part_1_valid(&self) -> bool {
        self.has_pair
    }

    fn part_2_valid(&self) -> bool {
        self.has_true_pair || (self.streak == 2 && (self.number / 10_000) % 11 == 0)
    }
}

fn count_valid(context: &Context) -> (u32, u32) {
    let mut stack = Vec::new();
    let (mut count_1, mut count_2) = (0, 0);

    stack.push(State::init(context));
    while stack.len() > 0 {
        let state = stack.pop().unwrap();

        if state.magnitude < Context::MAX {
            stack.append(&mut state.next_states());
        } else if context.contains(state.number) {
            if state.part_1_valid() {
                count_1 += 1;
            }
            if state.part_2_valid() {
                count_2 += 1;
            }
        };
    }

    (count_1, count_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_states_1() {
        let context = Context::new(900_000, 999_999);
        let state = State::init(&context);
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
        let state = State::init(&context);
        let expected = vec![
            State {
                context: &context,
                number: 8,
                magnitude: 1,
                has_pair: false,
                has_true_pair: false,
                streak: 1,
            },
            State {
                context: &context,
                number: 9,
                magnitude: 1,
                has_pair: false,
                has_true_pair: false,
                streak: 1,
            },
        ];
        assert_eq!(expected, state.next_states());

        let expected_2 = vec![
            State {
                context: &context,
                number: 89,
                magnitude: 10,
                has_pair: false,
                has_true_pair: false,
                streak: 1,
            },
            State {
                context: &context,
                number: 99,
                magnitude: 10,
                has_pair: true,
                has_true_pair: false,
                streak: 2,
            },
        ];
        assert_eq!(expected_2, expected[1].next_states());
    }

    #[test]
    fn test_count_valid() {
        assert_eq!((1, 0), count_valid(&Context::new(111_111, 111_111)));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Some(Context::new(123456, 654321)),
            Context::parse("123456-654321")
        );
    }
}

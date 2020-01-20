use std::collections::VecDeque;

pub fn main(_puzzle_input: &String) {}

type Position = (i16, i16);

#[derive(Debug, PartialEq)]
enum Direction { Up, Right, Down, Left }

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: u16,
}

impl Move {
    fn parse(string: &str) -> Option<Move> {
        let mut chars = string.chars();

        let direction = match chars.next() {
            Some('U') => Direction::Up,
            Some('R') => Direction::Right,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            _         => return None,
        };

        let steps: String = chars.collect();
        let steps: u16 = match steps.parse() {
            Ok(value) => value,
            Err(_) => return None,
        };

        Some(Move { direction, steps })
    }
}

#[derive(Debug)]
struct Path {
    position: Position,
    moves: VecDeque<Move>,
    current_steps: u16,
}

impl Path {
    fn parse(string: &String) -> Path {
        let mut moves = VecDeque::new();
        for mv_str in string.split(",") {
            if let Some(mv) = Move::parse(mv_str) {
                moves.push_back(mv);
            }
        }
        Path { position: (0, 0), moves, current_steps: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path() {
        let input = String::from("U1,R1,D1,L1");
        let expected = vec![
            Move { direction: Direction::Up, steps: 1 },
            Move { direction: Direction::Right, steps: 1 },
            Move { direction: Direction::Down, steps: 1 },
            Move { direction: Direction::Left, steps: 1 },
        ];

        let path = Path::parse(&input);
        assert_eq!(expected, Vec::from(path.moves));
        assert_eq!(0, path.current_steps);
        assert_eq!((0, 0), path.position);
    }
}


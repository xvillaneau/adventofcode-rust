use std::collections::VecDeque;

pub fn main(_puzzle_input: &String) {}

#[derive(Debug, PartialEq)]
struct Pos(i16, i16);

#[derive(Debug, PartialEq)]
enum Direction { Up, Right, Down, Left }

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: u16,
}

impl Move {
    fn from_string(string: &str) -> Option<Move> {
        if string.len() < 2 { return None };
        let mut chars = string.chars();
        let direction = match chars.next().unwrap_or(' ') {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _   => return None,
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
    position: Pos,
    moves: VecDeque<Move>,
    current_steps: u16,
}

impl Path {
    fn from_string(string: &String) -> Path {
        let mut moves = VecDeque::new();
        for mv_str in string.split(",") {
            if let Some(mv) = Move::from_string(mv_str) {
                moves.push_back(mv);
            }
        }
        Path { position: Pos(0, 0), moves, current_steps: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_path() {
        let input = String::from("U1,R1,D1,L1");
        let expected = vec![
            Move { direction: Direction::Up, steps: 1 },
            Move { direction: Direction::Right, steps: 1 },
            Move { direction: Direction::Down, steps: 1 },
            Move { direction: Direction::Left, steps: 1 },
        ];

        let path = Path::from_string(&input);
        assert_eq!(expected, Vec::from(path.moves));
        assert_eq!(0, path.current_steps);
        assert_eq!(Pos(0, 0), path.position);
    }
}


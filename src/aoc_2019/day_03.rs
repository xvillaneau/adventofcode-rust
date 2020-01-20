use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main(puzzle_input: &String) {
    let (mut fst, mut snd) = Path::parse_puzzle_input(puzzle_input).unwrap();

    let posmap_fst = fst.collect_positions();
    let posmap_snd = snd.collect_positions();
    let inter = intersections(&posmap_fst, &posmap_snd);
   
    println!("Part 1: {}", closest_distance(&inter).unwrap());
    println!("Part 2: {}", shortest_path(&inter).unwrap());

}

type Position = (i16, i16);
type PosMap = HashMap<Position, u16>;

#[derive(Debug, PartialEq)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn step(&self, position: Position) -> Position {
        let (x, y) = position;
        match self {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }
}

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
    fn parse(string: &str) -> Path {
        let mut moves = VecDeque::new();
        for mv_str in string.split(",") {
            if let Some(mv) = Move::parse(mv_str) {
                moves.push_back(mv);
            }
        }
        Path { position: (0, 0), moves, current_steps: 0 }
    }

    fn parse_puzzle_input(string: &str) -> Option<(Path, Path)> {
        let mut lines = string.lines();
        
        let first = match lines.next() {
            Some(line) => Path::parse(&line),
            None => return None,
        };
        let second = match lines.next() {
            Some(line) => Path::parse(&line),
            None => return None,
        };
        Some((first, second))
    }

    fn collect_positions(&mut self) -> PosMap {
        let mut pos_counts = HashMap::new();
        
        for (i, pos) in self.enumerate() {
            pos_counts.entry(pos).or_insert((i + 1) as u16 );
        };

        pos_counts
    }
}

impl Iterator for Path {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moves.len() == 0 { return None; };

        let cur_move = &self.moves[0];
        self.position = cur_move.direction.step(self.position);

        self.current_steps += 1;
        if self.current_steps == cur_move.steps {
            self.current_steps = 0;
            self.moves.pop_front();
        }

        Some(self.position)
    }
}

fn intersections(first: &PosMap, second: &PosMap) -> PosMap {
    let mut intersections: PosMap = HashMap::new();
    for (pos, steps_fst) in first.iter() {
        if let Some(steps_snd) = second.get(pos) {
            intersections.insert(*pos, steps_fst + steps_snd);
        }
    };
    intersections
}

fn closest_distance(intersections: &PosMap) -> Option<u16> {
    intersections.keys()
        .map(|pos| {pos.0.abs() as u16 + pos.1.abs() as u16})
        .min()
}

fn shortest_path(intersections: &PosMap) -> Option<u16> {
    intersections.values().min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "U1,R1,D1,L1";

    #[test]
    fn test_parse_path() {
        let expected = vec![
            Move { direction: Direction::Up, steps: 1 },
            Move { direction: Direction::Right, steps: 1 },
            Move { direction: Direction::Down, steps: 1 },
            Move { direction: Direction::Left, steps: 1 },
        ];

        let path = Path::parse(TEST_PATH);
        assert_eq!(expected, Vec::from(path.moves));
        assert_eq!(0, path.current_steps);
        assert_eq!((0, 0), path.position);
    }

    #[test]
    fn test_parse_puzzle_input() {
        let (fst, snd) = Path::parse_puzzle_input("U1\nD1\n").unwrap();

        let fst_mv = Vec::from(fst.moves);
        let snd_mv = Vec::from(snd.moves);
        assert_eq!(vec![Move { direction: Direction::Up, steps: 1 }], fst_mv);
        assert_eq!(vec![Move { direction: Direction::Down, steps: 1 }], snd_mv);
                   
    }

    #[test]
    fn test_iter_path_1() {
        let expected = vec![(0, 1), (1, 1), (1, 0), (0, 0)];

        let path = Path::parse(TEST_PATH);
        let positions: Vec<Position> = path.collect();
        assert_eq!(expected, positions);
    }

    #[test]
    fn test_iter_path_2() {
        let expected = vec![(0, 1), (0, 2), (0, 3), (0, 4)];

        let path = Path::parse("U4");
        let positions: Vec<Position> = path.collect();
        assert_eq!(expected, positions);
    }

    #[test]
    fn test_collect_positions() {
        let mut expected = HashMap::new();
        expected.insert((0, 1), 1);
        expected.insert((0, 2), 2);
        expected.insert((1, 2), 3);
        expected.insert((1, 1), 4);
        expected.insert((-1, 1), 6);
        
        let mut path = Path::parse("U2,R1,D1,L2");
        assert_eq!(expected, path.collect_positions());
    }

    #[test]
    fn test_aoc_example_1() {
        let (mut path_1, mut path_2) = Path::parse_puzzle_input("\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
").unwrap();
        let inter = intersections(&path_1.collect_positions(), &path_2.collect_positions());

        assert_eq!(Some(159), closest_distance(&inter));
        assert_eq!(Some(610), shortest_path(&inter));
    }

    #[test]
    fn test_aoc_example_2() {
        let (mut path_1, mut path_2) = Path::parse_puzzle_input("\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
").unwrap();
        let inter = intersections(&path_1.collect_positions(), &path_2.collect_positions());

        assert_eq!(Some(135), closest_distance(&inter));
        assert_eq!(Some(410), shortest_path(&inter));
    }
}


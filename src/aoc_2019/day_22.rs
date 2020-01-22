pub fn main(puzzle_input: &str) {
    let techniques = parse_input(puzzle_input);

    let (deck_size, position) = (10007, 2019);
    println!("Part 1: {}", follow_card(&techniques, deck_size, position));
}

#[derive(PartialEq, Debug)]
enum Technique {
    DealNew,
    Cut(i64),
    DealIncrement(i64),
}

fn parse_line(line: &str) -> Option<Technique> {
    let line = line.trim();
    if line == "deal into new stack" {
        Some(Technique::DealNew)
    } else if line.starts_with("cut") {
        match line[4..].parse() {
            Ok(value) => Some(Technique::Cut(value)),
            Err(_) => None,
        }
    } else if line.starts_with("deal with increment") {
        match line[20..].parse() {
            Ok(value) => Some(Technique::DealIncrement(value)),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn parse_input(text: &str) -> Vec<Technique> {
    let mut techniques = Vec::new();
    for line in text.lines() {
        if let Some(tech) = parse_line(&line) {
            techniques.push(tech);
        }
    }
    techniques
}

fn move_card(technique: &Technique, deck_size: i64, position: i64) -> i64 {
    match technique {
        Technique::DealNew => (deck_size - position) - 1,
        Technique::Cut(cut) => (position - cut).rem_euclid(deck_size),
        Technique::DealIncrement(incr) => (position * incr).rem_euclid(deck_size),
    }
}

fn follow_card(techniques: &Vec<Technique>, deck_size: i64, position: i64) -> i64 {
    let mut position = position;
    for technique in techniques {
        position = move_card(technique, deck_size, position);
    }
    position
}

fn merge_techniques(techniques: &Vec<Technique>, deck_size: i64) -> [Technique; 2] {
    let pos_zero = follow_card(techniques, deck_size, 0);
    let pos_one = follow_card(techniques, deck_size, 1);
    let offset = deck_size - pos_zero;
    let increment = (pos_one - pos_zero).rem_euclid(deck_size);
    [Technique::DealIncrement(increment), Technique::Cut(offset)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let text = "\
            deal into new stack
            cut -2
            deal with increment 7
            cut 8
        ";
        let expected = vec![
            Technique::DealNew,
            Technique::Cut(-2),
            Technique::DealIncrement(7),
            Technique::Cut(8),
        ];
        assert_eq!(expected, parse_input(&text));
    }

    #[test]
    fn test_follow_card() {
        let moves = vec![
            Technique::Cut(6),
            Technique::DealIncrement(7),
            Technique::DealNew,
        ];
        let final_pos: Vec<i64> = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        for (end, start) in final_pos.iter().enumerate() {
            assert_eq!(end as i64, follow_card(&moves, 10, *start));
        }
    }
}

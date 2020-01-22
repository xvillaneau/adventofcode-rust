use modinverse;

pub fn main(puzzle_input: &str) {
    let techniques = parse_input(puzzle_input);

    let (deck_size, position) = (10007, 2019);
    println!("Part 1: {}", follow_card(&techniques, deck_size, position));

    let (deck_size, repeats, position) = (119_315_717_514_047, 101_741_582_076_661, 2020);
    let merged = merge_techniques_n_times(&techniques, deck_size, repeats);
    println!("Part 2: {}", reverse_follow_card(&merged, deck_size, position));
}

#[derive(PartialEq, Debug, Clone)]
enum Technique {
    DealNew,
    Cut(i128),
    DealIncrement(i128),
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

fn move_card(technique: &Technique, deck_size: i128, position: i128) -> i128 {
    match technique {
        Technique::DealNew => (deck_size - position) - 1,
        Technique::Cut(cut) => (position - cut).rem_euclid(deck_size),
        Technique::DealIncrement(incr) => (position * incr).rem_euclid(deck_size),
    }
}

fn follow_card(techniques: &Vec<Technique>, deck_size: i128, position: i128) -> i128 {
    let mut position = position;
    for technique in techniques {
        position = move_card(technique, deck_size, position);
    }
    position
}

fn reverse_move_card(technique: &Technique, deck_size: i128, position: i128) -> i128 {
    match technique {
        Technique::DealNew => (deck_size - position) - 1,
        Technique::Cut(cut) => (position + cut).rem_euclid(deck_size),
        Technique::DealIncrement(incr) => {
            let (_, p, _) = modinverse::egcd(*incr, deck_size);
            (position * p).rem_euclid(deck_size)
        }
    }
}

fn reverse_follow_card(techniques: &Vec<Technique>, deck_size: i128, position: i128) -> i128 {
    let mut position = position;
    for technique in techniques.iter().rev() {
        position = reverse_move_card(technique, deck_size, position);
    }
    position
}

fn merge_techniques(techniques: &Vec<Technique>, deck_size: i128) -> Vec<Technique> {
    let pos_zero = follow_card(techniques, deck_size, 0);
    let pos_one = follow_card(techniques, deck_size, 1);
    let offset = deck_size - pos_zero;
    let increment = (pos_one - pos_zero).rem_euclid(deck_size);
    vec![Technique::DealIncrement(increment), Technique::Cut(offset)]
}

fn merge_techniques_n_times(techniques: &Vec<Technique>, deck_size: i128, repeats: i128) -> Vec<Technique> {
    let mut components = Vec::new();
    let mut repeats = repeats;
    let mut base = merge_techniques(techniques, deck_size);
    
    while repeats > 0 {
        if repeats & 1 == 1 {
            components.append(&mut base.clone());
        }
        base.append(&mut base.clone());
        base = merge_techniques(&base, deck_size);
        repeats >>= 1;
    }
    merge_techniques(&components, deck_size)
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
        let final_pos: Vec<i128> = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        for (end, start) in final_pos.iter().enumerate() {
            assert_eq!(end as i128, follow_card(&moves, 10, *start));
        }
    }

    #[test]
    fn test_reverse_follow_card() {
        let moves = vec![
            Technique::Cut(6),
            Technique::DealIncrement(7),
            Technique::DealNew,
        ];
        let end_pos: Vec<i128> = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        for (end, start) in end_pos.iter().enumerate() {
            assert_eq!(*start, reverse_follow_card(&moves, 10, end as i128));
        }
    }

    #[test]
    fn test_merge_techniques() {
        let techniques = vec![
            Technique::DealNew,
            Technique::Cut(-2),
            Technique::DealIncrement(7),
            Technique::Cut(8),
            Technique::Cut(-4),
            Technique::DealIncrement(7),
            Technique::Cut(3),
            Technique::DealIncrement(9),
            Technique::DealIncrement(3),
            Technique::Cut(-1),
        ];
        let repeats = 13;
        let size = 10;

        let mut repeat_techniques = Vec::new();
        for _ in 0..repeats {
            repeat_techniques.append(&mut techniques.clone());
        }
        let merged_techniques = merge_techniques_n_times(&techniques, size, repeats);

        for i in 0..size {
            assert_eq!(follow_card(&repeat_techniques, size, i), follow_card(&merged_techniques, size, i));
        }
    }
}

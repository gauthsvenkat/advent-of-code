use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

type Pos = (u8, u8);
type Keypad = HashMap<Pos, char>;

fn mv(current: &Pos, dir: char, keypad: &Keypad) -> Pos {
    let (x, y) = current;

    let maybe_next = match dir {
        'U' => (x.saturating_sub(1), *y),
        'D' => (x.saturating_add(1), *y),
        'L' => (*x, y.saturating_sub(1)),
        'R' => (*x, y.saturating_add(1)),
        c => panic!("Invalid direction {c}!"),
    };

    if keypad.contains_key(&maybe_next) {
        maybe_next
    } else {
        *current
    }
}

fn follow(current: &Pos, instruction: &str, keypad: &Keypad) -> Pos {
    let mut pos = *current;

    for c in instruction.chars() {
        pos = mv(&pos, c, keypad);
    }

    pos
}

fn get_code(mut pos: Pos, instructions: &[String], keypad: &Keypad) -> String {
    let mut code = String::new();

    for instruction in instructions.iter() {
        pos = follow(&pos, instruction, keypad);

        let digit = *keypad.get(&pos).unwrap();

        code.push(digit);
    }

    code
}

fn p1(input: &str) -> String {
    let instructions = parse(input);

    let keypad: Keypad = HashMap::from([
        ((0, 0), '1'),
        ((0, 1), '2'),
        ((0, 2), '3'),
        ((1, 0), '4'),
        ((1, 1), '5'),
        ((1, 2), '6'),
        ((2, 0), '7'),
        ((2, 1), '8'),
        ((2, 2), '9'),
    ]);

    get_code((1, 1), &instructions, &keypad)
}

fn p2(input: &str) -> String {
    let instructions = parse(input);

    let keypad: Keypad = HashMap::from([
        ((0, 2), '1'),
        ((1, 1), '2'),
        ((1, 2), '3'),
        ((1, 3), '4'),
        ((2, 0), '5'),
        ((2, 1), '6'),
        ((2, 2), '7'),
        ((2, 3), '8'),
        ((2, 4), '9'),
        ((3, 1), 'A'),
        ((3, 2), 'B'),
        ((3, 3), 'C'),
        ((4, 2), 'D'),
    ]);

    get_code((2, 0), &instructions, &keypad)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input)),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), "1985");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), "5DB3");
    }
}

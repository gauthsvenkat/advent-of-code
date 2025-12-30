use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

type Position = (isize, isize);

fn visit(directions: &[char]) -> HashSet<Position> {
    let (mut x, mut y) = (0, 0);
    let mut visited = HashSet::from([(x, y)]);

    for dir in directions {
        (x, y) = match dir {
            '>' => (x + 1, y),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '^' => (x, y - 1),
            c => panic!("Invalid direction {c}!"),
        };

        visited.insert((x, y));
    }

    visited
}

fn p1(input: &str) -> usize {
    visit(&parse(input)).len()
}

fn p2(input: &str) -> usize {
    let directions = parse(input);

    let mut santa = Vec::new();
    let mut robo = Vec::new();

    for d in directions.chunks(2) {
        santa.push(d[0]);
        robo.push(d[1]);
    }

    let mut visited = visit(&santa);
    visited.extend(visit(&robo));

    visited.len()
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

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");
    const EXAMPLE3: &str = include_str!("../eg3.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 2);
        assert_eq!(p1(EXAMPLE2), 4);
        assert_eq!(p1(EXAMPLE3), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), 3);
        assert_eq!(p2(EXAMPLE3), 11);
    }
}

use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;

type Position = (usize, usize);

fn parse(input: &str) -> HashSet<Position> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn area((x1, y1): &Position, (x2, y2): &Position) -> usize {
    (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1)
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_combinations()
        .map(|(r1, r2)| area(r1, r2))
        .max()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let positions = parse(input);

    positions
        .iter()
        .tuple_combinations()
        .filter(|((x1, y1), (x2, y2))| {
            let p3 = (*x1, *y2);
            let p4 = (*x2, *y1);

            positions.contains(&p3) && positions.contains(&p4)
        })
        .map(|(r1, r2)| area(r1, r2))
        .max()
        .unwrap()
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 50);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 24);
    }
}

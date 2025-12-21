use std::env;
use std::fs;

fn parse(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let (opp, you) = line.split_once(' ').unwrap();

            (opp.chars().next().unwrap(), you.chars().next().unwrap())
        })
        .collect()
}

fn score(opp: char, you: char) -> usize {
    let score_winning = match (opp, you) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0,
    };

    let score_shape = match you {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!("Character {you} isn't part of the specifications"),
    };

    score_winning + score_shape
}

fn p1(input: &str) -> usize {
    parse(input).iter().map(|(o, y)| score(*o, *y)).sum()
}

fn choose(opp: char, how: char) -> char {
    match (opp, how) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'X', // rock
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'Y', // paper
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'Z', // scissor
        _ => unreachable!("State {opp} {how} should not be possible"),
    }
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|(o, h)| (*o, choose(*o, *h)))
        .map(|(o, y)| score(o, y))
        .sum()
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
        assert_eq!(p1(EXAMPLE), 15);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 12);
    }
}

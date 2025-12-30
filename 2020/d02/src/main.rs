use std::{env, fs};

#[derive(Debug)]
struct Requirement {
    letter: char,
    n_min: usize,
    n_max: usize,
}

fn parse(input: &str) -> Vec<(Requirement, String)> {
    input
        .lines()
        .map(|line| {
            let (requirement, password) = line.split_once(':').unwrap();
            let (range, letter) = requirement.split_once(' ').unwrap();
            let (n_min, n_max) = range.split_once('-').unwrap();

            let password = password.trim().to_string();

            let n_min: usize = n_min.parse().unwrap();
            let n_max: usize = n_max.parse().unwrap();
            let letter = letter.trim().chars().next().unwrap();

            (
                Requirement {
                    letter,
                    n_min,
                    n_max,
                },
                password,
            )
        })
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(req, password)| {
            let n = password.chars().filter(|&c| c == req.letter).count();

            (req.n_min <= n) && (n <= req.n_max)
        })
        .count()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(req, password)| {
            let c1 = password.chars().nth(req.n_min - 1).unwrap();
            let c2 = password.chars().nth(req.n_max - 1).unwrap();

            (c1 == req.letter) ^ (c2 == req.letter)
        })
        .count()
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
        assert_eq!(p1(EXAMPLE), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 1);
    }
}

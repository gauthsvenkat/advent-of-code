use itertools::Itertools;
use std::{env, fs};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|row| {
            let largest = row.iter().max().unwrap();
            let smallest = row.iter().min().unwrap();

            largest.abs_diff(*smallest)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|row| {
            row.iter()
                .tuple_combinations()
                .find_map(|(a, b)| {
                    if a.is_multiple_of(*b) {
                        Some(a / b)
                    } else if b.is_multiple_of(*a) {
                        Some(b / a)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
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

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 18);
        assert_eq!(p1(EXAMPLE2), 18);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), 9);
    }
}

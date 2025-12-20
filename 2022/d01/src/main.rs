use std::collections::HashSet;
use std::env;
use std::fs;

use itertools::Itertools;

type Food = HashSet<usize>;

fn parse(input: &str) -> Vec<Food> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|c| c.parse::<usize>().unwrap()).collect())
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|food| food.iter().sum())
        .max()
        .unwrap()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|food| food.iter().sum::<usize>())
        .k_largest(3)
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
        assert_eq!(p1(EXAMPLE), 24000);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 45000);
    }
}

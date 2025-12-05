use std::env;
use std::fs;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_number(line: &str) -> usize {
    let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
    let last_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap();

    format!("{}{}", first_digit, last_digit).parse().unwrap()
}

fn p1(input: &str) -> usize {
    parse(input).into_iter().map(|line| get_number(&line)).sum()
}

fn p2(input: &str) -> usize {
    let lines = parse(input);

    lines
        .into_iter()
        .map(|mut line| {
            let replacements = vec![
                ("one", "o1e"),
                ("two", "t2o"),
                ("three", "t3e"),
                ("four", "4"),
                ("five", "5e"),
                ("six", "6"),
                ("seven", "7n"),
                ("eight", "e8t"),
                ("nine", "n9e"),
            ];

            for (old, new) in replacements {
                line = line.replace(old, new);
            }

            get_number(&line)
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
    fn test_p1_example1() {
        assert_eq!(p1(EXAMPLE1), 142);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(p2(EXAMPLE2), 281);
    }
}

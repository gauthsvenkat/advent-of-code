use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.to_string()).collect())
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|group| {
            let unique_answers: HashSet<char> = group.iter().flat_map(|q| q.chars()).collect();
            unique_answers.len()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|q| q.chars().collect::<HashSet<char>>())
                .reduce(|acc, q| acc.intersection(&q).copied().collect())
                .map_or(0, |s| s.len())
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
        assert_eq!(p1(EXAMPLE1), 6);
        assert_eq!(p1(EXAMPLE2), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 3);
        assert_eq!(p2(EXAMPLE2), 6);
    }
}

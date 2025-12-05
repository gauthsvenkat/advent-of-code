use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let available: Vec<String> = groups[0].split(',').map(|s| s.trim().to_string()).collect();

    let designs: Vec<String> = groups[1].lines().map(|s| s.to_string()).collect();

    (available, designs)
}

fn all_possible(target: &str, words: &[String], cache: &mut HashMap<String, usize>) -> usize {
    if target.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(target) {
        return count;
    }

    let mut count = 0;

    for word in words {
        if target.starts_with(word) {
            count += all_possible(&target[word.len()..], words, cache);
        }
    }

    *cache.entry(target.to_string()).or_insert(count)
}

fn p1(input: &str) -> usize {
    let (available_towels, designs) = parse(input);

    designs
        .iter()
        .filter(|d| all_possible(d, &available_towels, &mut HashMap::new()) > 0)
        .count()
}

fn p2(input: &str) -> usize {
    let (available_towels, designs) = parse(input);

    designs
        .iter()
        .map(|d| all_possible(d, &available_towels, &mut HashMap::new()))
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
        assert_eq!(p1(EXAMPLE), 6);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 16);
    }
}

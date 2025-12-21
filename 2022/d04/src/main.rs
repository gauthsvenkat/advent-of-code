use std::env;
use std::fs;
use std::ops::RangeInclusive;

type IDRange = RangeInclusive<usize>;

fn parse_range(input: &str) -> IDRange {
    let (start, end) = input.split_once('-').unwrap();

    let start: usize = start.parse().unwrap();
    let end: usize = end.parse().unwrap();

    start..=end
}

fn parse(input: &str) -> Vec<(IDRange, IDRange)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();

            (parse_range(first), parse_range(second))
        })
        .collect()
}

enum Overlap {
    None,
    Full,
    Partial,
}

fn determine_overlap(a: &IDRange, b: &IDRange) -> Overlap {
    // switching a and b the following way allows us to collapse
    // `is_overlapping` to a few cases, that are symmetric.
    let (a, b) = if (a.start(), b.end()) < (b.start(), a.end()) {
        (a, b)
    } else {
        (b, a)
    };

    if !a.contains(b.start()) && !a.contains(b.end()) {
        Overlap::None
    } else if a.contains(b.start()) && a.contains(b.end()) {
        Overlap::Full
    } else if a.contains(b.start()) && !a.contains(b.end()) {
        Overlap::Partial
    } else {
        panic!("Couldn't determine overlap for ranges {:?} and {:?}!", a, b)
    }
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(a, b)| matches!(determine_overlap(a, b), Overlap::Full))
        .count()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(a, b)| matches!(determine_overlap(a, b), Overlap::Full | Overlap::Partial))
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 2);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 4);
    }
}

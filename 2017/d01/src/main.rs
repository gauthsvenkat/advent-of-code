use itertools::Itertools;
use std::{env, fs};

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn p1(input: &str) -> u32 {
    parse(input)
        .iter()
        .circular_tuple_windows()
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

fn p2(input: &str) -> u32 {
    let digits = parse(input);

    let mid = digits.len() / 2;

    let (first, second) = (digits.iter().take(mid), digits.iter().skip(mid));

    first
        .zip(second)
        .filter_map(|(a, b)| if a == b { Some(a + b) } else { None })
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
    const EXAMPLE3: &str = include_str!("../eg3.txt");
    const EXAMPLE4: &str = include_str!("../eg4.txt");
    const EXAMPLE5: &str = include_str!("../eg5.txt");
    const EXAMPLE6: &str = include_str!("../eg6.txt");
    const EXAMPLE7: &str = include_str!("../eg7.txt");
    const EXAMPLE8: &str = include_str!("../eg8.txt");
    const EXAMPLE9: &str = include_str!("../eg9.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 3);
        assert_eq!(p1(EXAMPLE2), 4);
        assert_eq!(p1(EXAMPLE3), 0);
        assert_eq!(p1(EXAMPLE4), 9);
        assert_eq!(p1(EXAMPLE5), 0);
        assert_eq!(p1(EXAMPLE6), 3);
        assert_eq!(p1(EXAMPLE7), 0);
        assert_eq!(p1(EXAMPLE8), 0);
        assert_eq!(p1(EXAMPLE9), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 0);
        assert_eq!(p2(EXAMPLE2), 4);
        assert_eq!(p2(EXAMPLE3), 0);
        assert_eq!(p2(EXAMPLE4), 6);
        assert_eq!(p2(EXAMPLE5), 6);
        assert_eq!(p2(EXAMPLE6), 0);
        assert_eq!(p2(EXAMPLE7), 4);
        assert_eq!(p2(EXAMPLE8), 12);
        assert_eq!(p2(EXAMPLE9), 4);
    }
}

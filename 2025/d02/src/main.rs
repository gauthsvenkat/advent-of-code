use std::env;
use std::fs;
use std::panic;

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .split(",")
        .map(|range| {
            let (lb, ub) = range.trim().split_once("-").unwrap();
            (lb.parse().unwrap(), ub.parse().unwrap())
        })
        .collect()
}

// Helper function to split a string perfectly into chunks
// Return None if not possible
fn _split_str_in(s: &str, size: usize) -> Option<Vec<&str>> {
    let l = s.len();

    if size >= l {
        return None;
    }

    if !l.is_multiple_of(size) {
        return None;
    }

    Some(
        s.as_bytes()
            .chunks(size)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect(),
    )
}

fn _is_num_digits_even(num_str: &str) -> bool {
    num_str.len().rem_euclid(2) == 0
}

fn _is_invalid_v1(num: &usize) -> bool {
    let num_str = num.to_string();
    let mid = num_str.len().div_ceil(2);

    if let Some(splitted) = _split_str_in(&num_str, mid) {
        splitted[0] == splitted[1]
    } else {
        false
    }
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);

    parsed_input
        .iter()
        .map(|(lb, ub)| (*lb..=*ub).filter(_is_invalid_v1).sum::<usize>())
        .sum()
}

fn _is_invalid_v2(num: &usize) -> bool {
    let num_str = num.to_string();
    let mid = num_str.len().div_ceil(2);

    for size in 1..(mid + 1) {
        if let Some(splitted) = _split_str_in(&num_str, size) {
            if splitted.windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }
    }

    false
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);

    parsed_input
        .iter()
        .map(|(lb, ub)| (*lb..=*ub).filter(_is_invalid_v2).sum::<usize>())
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
        assert_eq!(p1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 4174379265);
    }
}

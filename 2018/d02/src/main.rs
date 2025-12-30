use itertools::Itertools;
use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn contains_char_of_len(box_id: &str, n: usize) -> bool {
    let unique_chars: HashSet<char> = box_id.chars().collect();

    for ch in unique_chars {
        if box_id.chars().filter(|&c| c == ch).count() == n {
            return true;
        }
    }

    false
}

fn p1(input: &str) -> usize {
    let box_ids = parse(input);

    let n_twos = box_ids
        .iter()
        .filter(|box_id| contains_char_of_len(box_id, 2))
        .count();

    let n_threes = box_ids
        .iter()
        .filter(|box_id| contains_char_of_len(box_id, 3))
        .count();

    n_twos * n_threes
}

fn differing_chars(id1: &str, id2: &str) -> Option<usize> {
    let mut violation_found = false;

    let mut maybe = None;

    for (i, (a, b)) in id1.chars().zip(id2.chars()).enumerate() {
        if a != b {
            // if a != b but we've already found a violation
            // before, this pair probably ain't it.
            if violation_found {
                return None;
            } else {
                violation_found = true;
                maybe = Some(i);
            }
        }
    }

    maybe
}

fn p2(input: &str) -> String {
    parse(input)
        .iter()
        .tuple_combinations()
        .find_map(|(id1, id2)| {
            differing_chars(id1, id2).map(|idx| {
                id1.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if i != idx { Some(c) } else { None })
                    .collect()
            })
        })
        .unwrap()
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
        assert_eq!(p1(EXAMPLE1), 12);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), "fgij".to_string());
    }
}

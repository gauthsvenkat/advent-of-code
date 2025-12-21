use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn char_to_num(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => panic!("Invalid character - {c}"),
    }
}

fn find_common(sacks: &[String]) -> char {
    let mut common: HashSet<char> = sacks.first().unwrap().chars().collect();

    for sack in sacks {
        let unique: HashSet<char> = sack.chars().collect();
        common = common.intersection(&unique).copied().collect();
    }

    common.into_iter().next().unwrap()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|rucksack| {
            let mid = rucksack.len() / 2;
            let (first, second) = rucksack.split_at(mid);

            find_common(&[first.to_string(), second.to_string()])
        })
        .map(char_to_num)
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .chunks(3)
        .map(find_common)
        .map(char_to_num)
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
        assert_eq!(p1(EXAMPLE), 157);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 70);
    }
}

use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_joltage(bank: &[usize], num_batteries: usize) -> Option<usize> {
    if num_batteries == 0 {
        return Some(0);
    }

    if num_batteries > bank.len() {
        return None;
    }

    let (idx, max) = bank[..=bank.len() - num_batteries]
        .iter()
        .enumerate()
        .max_by(|(i, &a), (j, &b)| a.cmp(&b).then(j.cmp(i)))
        .unwrap();

    get_joltage(&bank[idx + 1..], num_batteries - 1)
        .map(|joltage| max * 10_usize.pow(num_batteries as u32 - 1) + joltage)
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);
    parsed_input
        .iter()
        .map(|bank| get_joltage(bank, 2).unwrap())
        .sum()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    parsed_input
        .iter()
        .map(|bank| get_joltage(bank, 12).unwrap())
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
        assert_eq!(p1(EXAMPLE), 357);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 3121910778619);
    }
}

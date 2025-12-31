use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn count(messages: &[String], col_idx: usize, most_common: bool) -> char {
    let mut counter: HashMap<char, usize> = HashMap::new();

    messages
        .iter()
        .map(|s| s.chars().nth(col_idx).unwrap())
        .for_each(|c| *counter.entry(c).or_default() += 1);

    *counter
        .iter()
        .max_by(|(_, v1), (_, v2)| if most_common { v1.cmp(v2) } else { v2.cmp(v1) })
        .map(|(c, _)| c)
        .unwrap()
}

fn p1(input: &str) -> String {
    let messages = parse(input);
    let mut ecc_message = String::new();

    for i in 0..messages[0].len() {
        ecc_message.push(count(&messages, i, true));
    }

    ecc_message
}

fn p2(input: &str) -> String {
    let messages = parse(input);
    let mut ecc_message = String::new();

    for i in 0..messages[0].len() {
        ecc_message.push(count(&messages, i, false));
    }

    ecc_message
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
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), "easter");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), "advent");
    }
}

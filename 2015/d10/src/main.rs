use std::{env, fs, str::from_utf8};

fn parse(input: &str) -> String {
    input.trim().to_string()
}

fn group(input: &str) -> Vec<&str> {
    input
        .as_bytes()
        .chunk_by(|a, b| a == b)
        .map(|chunk| from_utf8(chunk).unwrap())
        .collect()
}

fn look_and_say(input: &[&str]) -> String {
    let mut next = String::new();

    for s in input {
        let n = s.len();
        let c = s.chars().next().unwrap();

        next.push_str(&format!("{n}{c}"));
    }

    next
}

fn process(input: String, n: usize) -> usize {
    let mut next = input;

    for _ in 0..n {
        next = look_and_say(&group(&next))
    }

    next.len()
}

fn p1(input: &str) -> usize {
    process(parse(input), 40)
}

fn p2(input: &str) -> usize {
    process(parse(input), 50)
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
        assert_eq!(p1(EXAMPLE), 82350);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 1166642);
    }
}

use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_number(line: &Vec<char>) -> usize {
    let first_digit = line.iter().find(|c| c.is_numeric()).unwrap();
    let last_digit = line.iter().rev().find(|c| c.is_numeric()).unwrap();

    format!("{}{}", first_digit, last_digit).parse().unwrap()
}

fn p1(input: &str) -> usize {
    let lines = parse(input);

    lines.iter().map(get_number).sum()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    todo!()
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

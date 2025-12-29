use std::{env, fs};

use Command::*;

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (command, unit) = line.split_once(' ').unwrap();
            let unit: usize = unit.parse().unwrap();

            match command {
                "forward" => Forward(unit),
                "down" => Down(unit),
                "up" => Up(unit),
                s => panic!("Invalid command: {s}"),
            }
        })
        .collect()
}

fn p1(input: &str) -> usize {
    let commands = parse(input);

    let mut h_pos = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Forward(unit) => h_pos += unit,
            Down(unit) => depth += unit,
            Up(unit) => depth -= unit,
        }
    }

    h_pos * depth
}

fn p2(input: &str) -> usize {
    let commands = parse(input);

    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Forward(unit) => {
                h_pos += unit;
                depth += aim * unit;
            }
            Down(unit) => aim += unit,
            Up(unit) => aim -= unit,
        }
    }

    h_pos * depth
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
        assert_eq!(p1(EXAMPLE), 150);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 900);
    }
}

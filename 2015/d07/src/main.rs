use std::{collections::HashMap, env, fs};

use Instruction::*;
use Signal::*;

#[derive(Debug)]
enum Signal {
    Literal(usize),
    Wire(String),
}

#[derive(Debug)]
enum Instruction {
    Provide(Signal),
    And(Signal, String),
    OR(String, String),
    Not(String),
    Lshift(String, u8),
    Rshift(String, u8),
}

impl Instruction {
    fn from(input: &str) -> Instruction {
        let input: Vec<&str> = input.split_whitespace().collect();

        let to_signal = || {
            if input[0].chars().next().unwrap().is_ascii_digit() {
                Literal(input[0].parse().unwrap())
            } else {
                Wire(input[0].to_string())
            }
        };

        match input.len() {
            1 => Provide(to_signal()),
            2 => Not(input[1].to_string()),
            3 => match input[1] {
                "AND" => And(to_signal(), input[2].to_string()),
                "OR" => OR(input[0].to_string(), input[2].to_string()),
                "LSHIFT" => Lshift(input[0].to_string(), input[2].parse().unwrap()),
                "RSHIFT" => Rshift(input[0].to_string(), input[2].parse().unwrap()),
                s => panic!("Invalid instruction {s}!"),
            },
            n => panic!("Invalid length {n} for {:?}!", input),
        }
    }
}

fn parse(input: &str) -> HashMap<String, Instruction> {
    input
        .lines()
        .map(|line| {
            let (instruction, wire) = line.split_once(" -> ").unwrap();
            let instruction = Instruction::from(instruction.trim());
            let wire = wire.trim().to_string();

            (wire, instruction)
        })
        .collect()
}

fn resolve(
    wire: &str,
    connections: &HashMap<String, Instruction>,
    resolved_map: &mut HashMap<String, usize>,
) -> usize {
    if let Some(val) = resolved_map.get(wire) {
        return *val;
    }

    let wire = wire.to_string();

    let instruction = connections.get(&wire).unwrap();

    let val = match instruction {
        Provide(Literal(val)) => *val,
        Provide(Wire(w)) => resolve(w, connections, resolved_map),
        And(Literal(val), b) => {
            let b = resolve(b, connections, resolved_map);

            val & b
        }
        And(Wire(a), b) => {
            let a = resolve(a, connections, resolved_map);
            let b = resolve(b, connections, resolved_map);

            a & b
        }
        OR(a, b) => {
            let a = resolve(a, connections, resolved_map);
            let b = resolve(b, connections, resolved_map);

            a | b
        }
        Not(a) => !resolve(a, connections, resolved_map),
        Lshift(a, n) => resolve(a, connections, resolved_map) << n,
        Rshift(a, n) => resolve(a, connections, resolved_map) >> n,
    };

    resolved_map.insert(wire, val);

    val
}

fn p1(input: &str) -> usize {
    let connections = parse(input);
    resolve("a", &connections, &mut HashMap::new())
}

fn p2(input: &str) -> usize {
    let mut connections = parse(input);
    let val = resolve("a", &connections, &mut HashMap::new());
    *connections.get_mut("b").unwrap() = Provide(Literal(val));
    resolve("a", &connections, &mut HashMap::new())
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
        assert_eq!(p1(EXAMPLE), 72);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 72);
    }
}

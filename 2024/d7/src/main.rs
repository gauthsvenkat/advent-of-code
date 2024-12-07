use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut equations: HashMap<u64, Vec<u64>> = HashMap::new();

    for line in input.lines() {
        if let Some((key_str, value_str)) = line.split_once(':') {
            let key: u64 = key_str.trim().parse().unwrap();
            let values: Vec<u64> = value_str
                .split_whitespace()
                .map(|v| v.trim().parse().unwrap())
                .collect();

            equations.insert(key, values);
        }
    }

    equations
}

fn calibrate(eq: &Vec<u64>, target: u64, allowed_operators: &str, op_acc: Vec<char>) -> bool {
    if eq.len() - 1 != op_acc.len() {
        for op in allowed_operators.chars() {
            let mut new_operators = op_acc.clone();
            new_operators.push(op);
            if calibrate(eq, target, allowed_operators, new_operators) {
                return true;
            }
        }

        false
    } else {
        let mut result: u64 = eq[0];
        for i in 0..op_acc.len() {
            if result > target {
                return false;
            }
            match op_acc[i] {
                '*' => result *= eq[i + 1],
                '+' => result += eq[i + 1],
                '|' => {
                    let mut num = eq[i + 1];

                    while num > 0 {
                        result *= 10;
                        num /= 10;
                    }

                    result += eq[i + 1];
                }
                _ => panic!("Invalid operator"),
            }
        }

        result == target
    }
}

fn p1(input: &str) -> u64 {
    let equations = parse(input);
    let allowed_operators = "*+";

    equations
        .iter()
        .filter(|(target, eq)| calibrate(eq, **target, allowed_operators, Vec::new()))
        .map(|(target, _)| target)
        .sum()
}

fn p2(input: &str) -> u64 {
    let equations = parse(input);
    let allowed_operators = "*+|";

    equations
        .iter()
        .filter(|(target, eq)| calibrate(eq, **target, allowed_operators, Vec::new()))
        .map(|(target, _)| target)
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

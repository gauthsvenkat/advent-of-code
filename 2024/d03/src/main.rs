use regex::Regex;
use std::env;
use std::fs;

fn p1(input: &str) -> i32 {
    let mut sol: i32 = 0;

    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();

        sol += a * b;
    }

    sol
}

fn p2(input: &str) -> i32 {
    let mut sol: i32 = 0;

    let re = Regex::new(r"mul\(-?\d+,-?\d+\)|don't\(\)|do\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    let mut skip: bool = false;

    for m in re.find_iter(input).map(|c| c.as_str()) {
        match m {
            "do()" => {
                skip = false;
            }
            "don't()" => {
                skip = true;
            }
            _ => {
                if !skip {
                    let caps = mul_re.captures(m).unwrap();
                    let a: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                    let b: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                    sol += a * b;
                }
            }
        }
    }

    sol
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

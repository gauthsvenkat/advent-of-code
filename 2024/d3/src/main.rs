use regex::Regex;
use std::env;
use std::fs;

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path).unwrap()
}

fn p1(input: &str) -> i32 {
    let mut sol: i32 = 0;

    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
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

    for m in re.find_iter(&input).map(|c| c.as_str()) {
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
    let input = get_input();

    let p1_sol = p1(&input);
    println!("{p1_sol}");

    let p2_sol = p2(&input);
    println!("{p2_sol}");
}

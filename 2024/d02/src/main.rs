use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut levels: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            levels.push(num.parse().unwrap());
        }
        reports.push(levels);
    }

    reports
}

fn is_safe(report: &[i32]) -> bool {
    let mut p_delta: i32 = 0;

    for w in report.windows(2) {
        let c_delta: i32 = w[0] - w[1];
        let c_delta_abs = c_delta.abs();

        if !(1..=3).contains(&c_delta_abs) || ((c_delta * p_delta) < 0) {
            return false;
        }

        p_delta = c_delta;
    }

    true
}

fn p1(input: &str) -> u32 {
    let reports = parse(input);

    let mut num_safe: u32 = 0;

    for report in &reports {
        if is_safe(report) {
            num_safe += 1;
        }
    }

    num_safe
}

fn combinations_excluding_one(input: &[i32]) -> Vec<Vec<i32>> {
    let mut combinations = Vec::new();

    for i in 0..input.len() {
        let mut copy = input.to_owned();
        copy.remove(i);
        combinations.push(copy);
    }

    combinations
}

fn p2(input: &str) -> u32 {
    let reports = parse(input);

    let mut num_safe: u32 = 0;

    for report in &reports {
        if is_safe(report) {
            num_safe += 1;
        } else {
            for combi in combinations_excluding_one(report) {
                if is_safe(&combi) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }

    num_safe
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

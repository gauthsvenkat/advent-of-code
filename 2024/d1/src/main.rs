use std::collections::HashMap;
use std::env;
use std::fs;

fn readfile(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let line_content: Vec<&str> = line.split_whitespace().collect();

        let (a, b) = match &line_content[..] {
            [a, b] => {
                let a: i32 = a.parse().unwrap();
                let b: i32 = b.parse().unwrap();
                (a, b)
            }
            _ => panic!("Couldn't parse line: \"{line}\""),
        };

        l1.push(a);
        l2.push(b);
    }

    (l1, l2)
}

fn p1(input: &str) -> i32 {
    let (mut l1, mut l2) = parse(input);

    l1.sort();
    l2.sort();

    l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn p2(input: &str) -> i32 {
    let (l1, l2) = parse(input);

    let mut l2_counts = HashMap::new();
    for n in l2 {
        *l2_counts.entry(n).or_insert(0) += 1;
    }

    l1.iter().map(|n| n * l2_counts.get(n).unwrap_or(&0)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = readfile(filepath);

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input)),
        _ => panic!("Invalid part"),
    };
}

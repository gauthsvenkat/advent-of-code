use std::env;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn split_stone_if_even(num: usize) -> Option<(usize, usize)> {
    let mut temp = num;
    let mut num_digits = 0;

    while temp > 0 {
        temp /= 10;
        num_digits += 1;
    }

    if num_digits % 2 != 0 {
        return None;
    }

    let half = num_digits / 2;

    let divisor = 10usize.pow(half as u32);

    Some((num / divisor, num % divisor))
}

fn blink(arrangement: &[usize]) -> Vec<usize> {
    let mut new_arrangement: Vec<usize> = Vec::new();

    for &stone in arrangement.iter() {
        if stone == 0 {
            new_arrangement.push(1);
        } else if let Some((first, second)) = split_stone_if_even(stone) {
            new_arrangement.push(first);
            new_arrangement.push(second);
        } else {
            new_arrangement.push(stone * 2024);
        }
    }

    new_arrangement
}

fn p1(input: &str) -> usize {
    let mut arrangement = parse(input);

    for _ in 0..25 {
        arrangement = blink(&arrangement);
    }

    arrangement.len()
}

fn p2(input: &str) -> usize {
    let mut arrangement = parse(input);

    for i in 0..45 {
        // println!("{}: {}", i, arrangement.len());
        arrangement = blink(&arrangement);
    }

    arrangement.len()
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

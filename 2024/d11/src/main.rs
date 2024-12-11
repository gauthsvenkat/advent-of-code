use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn num_digits(mut num: usize) -> u32 {
    let mut num_digits = 0;

    while num > 0 {
        num /= 10;
        num_digits += 1;
    }

    num_digits
}

fn split_stone_if_even(num: usize) -> Option<(usize, usize)> {
    let n_digits = num_digits(num);

    if n_digits % 2 != 0 {
        return None;
    }

    let half = n_digits / 2;
    let divisor = 10usize.pow(half);

    Some((num / divisor, num % divisor))
}

fn grow(stone: usize, iteration: usize, record: &mut HashMap<(usize, usize), usize>) -> usize {
    if iteration == 1 {
        let length = if stone == 0 {
            1
        } else if num_digits(stone) % 2 == 0 {
            2
        } else {
            1
        };

        return *record.entry((stone, iteration)).or_insert(length);
    }

    if let Some(&value) = record.get(&(stone, iteration)) {
        return value;
    }

    let length = if stone == 0 {
        grow(1, iteration - 1, record)
    } else if let Some((first, second)) = split_stone_if_even(stone) {
        grow(first, iteration - 1, record) + grow(second, iteration - 1, record)
    } else {
        grow(stone * 2024, iteration - 1, record)
    };

    *record.entry((stone, iteration)).or_insert(length)
}

fn p1(input: &str) -> usize {
    let arrangement = parse(input);
    let mut record = HashMap::new();

    arrangement.iter().map(|&s| grow(s, 25, &mut record)).sum()
}

fn p2(input: &str) -> usize {
    let arrangement = parse(input);
    let mut record = HashMap::new();

    arrangement.iter().map(|&s| grow(s, 75, &mut record)).sum()
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

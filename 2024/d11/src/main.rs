use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(Copy, Clone)]
enum Split {
    One(usize),
    Two(usize, usize),
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

fn blink_stone(stone: usize, record: &mut HashMap<usize, Split>) -> Split {
    if let Some(&value) = record.get(&stone) {
        return value;
    }

    let value = if stone == 0 {
        Split::One(1)
    } else if let Some((first, second)) = split_stone_if_even(stone) {
        Split::Two(first, second)
    } else {
        Split::One(stone * 2024)
    };

    *record.entry(stone).or_insert(value)
}

fn blink_stones(stones: Vec<usize>, record: &mut HashMap<usize, Split>) -> Vec<usize> {
    let mut new_stones = Vec::new();

    for stone in stones {
        match blink_stone(stone, record) {
            Split::One(value) => new_stones.push(value),
            Split::Two(first, second) => {
                new_stones.push(first);
                new_stones.push(second);
            }
        }
    }

    new_stones
}

fn p1(input: &str) -> usize {
    let mut arrangement = parse(input);
    let mut record = HashMap::new();

    for _ in 0..25 {
        arrangement = blink_stones(arrangement, &mut record);
    }

    arrangement.len()
}

fn p2(input: &str) -> usize {
    let mut arrangement = parse(input);
    let mut record = HashMap::new();

    for i in 0..75 {
        println!("----Blink {}", i + 1);
        arrangement = blink_stones(arrangement, &mut record);
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

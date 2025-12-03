use cached::proc_macro::cached;
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

    if !n_digits.is_multiple_of(2) {
        return None;
    }

    let divisor = 10usize.pow(n_digits / 2);

    Some((num / divisor, num % divisor))
}

#[cached]
fn count(stone: usize, blink: usize) -> usize {
    if blink == 0 {
        1
    } else if stone == 0 {
        count(1, blink - 1)
    } else if let Some((first, second)) = split_stone_if_even(stone) {
        count(first, blink - 1) + count(second, blink - 1)
    } else {
        count(stone * 2024, blink - 1)
    }
}

fn p1(input: &str) -> usize {
    parse(input).iter().map(|&s| count(s, 25)).sum()
}

fn p2(input: &str) -> usize {
    parse(input).iter().map(|&s| count(s, 75)).sum()
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

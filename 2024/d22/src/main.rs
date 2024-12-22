use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn evolve(mut number: usize, times: usize, mut price_sequence: Vec<u8>) -> (usize, Vec<u8>) {
    price_sequence.push((number % 10) as u8);

    if times == 0 {
        return (number, price_sequence);
    }

    number = (number ^ (number << 6)) & ((1 << 24) - 1);
    number = (number ^ (number >> 5)) & ((1 << 24) - 1);
    number = (number ^ (number << 11)) & ((1 << 24) - 1);

    evolve(number, times - 1, price_sequence)
}

fn p1(input: &str) -> usize {
    let numbers = parse(input);
    numbers.iter().map(|&n| evolve(n, 2000, Vec::new()).0).sum()
}

fn get_bananas(price_sequence: &[u8], diff_sequence: &[i8], diff: &[i8]) -> Option<u8> {
    diff_sequence.windows(4).enumerate().find_map(|(i, w)| {
        if w == diff {
            Some(price_sequence[i + 4])
        } else {
            None
        }
    })
}

fn get_price_seq_to_diff_seq_map(price_sequences: &[Vec<u8>]) -> HashMap<Vec<u8>, Vec<i8>> {
    let mut price_to_diff_map = HashMap::new();

    price_sequences.iter().for_each(|ps| {
        let diff_sequence: Vec<i8> = ps.windows(2).map(|w| w[1] as i8 - w[0] as i8).collect();
        price_to_diff_map.insert(ps.to_vec(), diff_sequence);
    });

    price_to_diff_map
}

fn get_best_bananas(price_sequences: &[Vec<u8>]) -> usize {
    let mut best = 0;

    let price_to_diff_map = get_price_seq_to_diff_seq_map(price_sequences);

    let mut diff_cache = HashMap::new();

    for (i,price_sequence) in price_sequences.iter().enumerate() {
        println!("Processing price sequence {}", i);
        let diff_sequence: Vec<i8> = price_sequence
            .windows(2)
            .map(|w| w[1] as i8 - w[0] as i8)
            .collect();

        for diff in diff_sequence.windows(4) {
            if diff_cache.contains_key(diff) {
                continue;
            }

            let total = price_sequences
                .iter()
                .filter_map(|ps| get_bananas(ps, price_to_diff_map.get(ps).unwrap(), diff))
                .map(|b| b as usize)
                .sum();

            diff_cache.insert(diff.to_vec(), total);

            if total > best {
                best = total;
            }
        }
    }

    best
}

fn p2(input: &str) -> usize {
    let numbers = parse(input);
    let price_sequences: Vec<Vec<u8>> = numbers
        .iter()
        .map(|&n| evolve(n, 2000, Vec::new()).1)
        .collect();

    get_best_bananas(&price_sequences)
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

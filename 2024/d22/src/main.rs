use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

struct Buyer {
    final_secret: usize,
    diff_to_count_map: HashMap<(i8, i8, i8, i8), u8>,
}

fn evolve(initial_secret: usize, times: usize) -> Buyer {
    let mut price_sequence = Vec::new();
    let mut diff_sequence = Vec::new();
    let mut final_secret = initial_secret;

    let mut diff_to_count_map = HashMap::new();

    for i in 0..times {
        price_sequence.push((final_secret % 10) as u8);

        final_secret = (final_secret ^ (final_secret << 6)) & ((1 << 24) - 1);
        final_secret = (final_secret ^ (final_secret >> 5)) & ((1 << 24) - 1);
        final_secret = (final_secret ^ (final_secret << 11)) & ((1 << 24) - 1);

        if i > 0 {
            let (a, b) = (price_sequence[i - 1] as i8, price_sequence[i] as i8);
            diff_sequence.push(b - a);
        }

        if i > 3 {
            let diff = (
                diff_sequence[i - 4],
                diff_sequence[i - 3],
                diff_sequence[i - 2],
                diff_sequence[i - 1],
            );

            diff_to_count_map.entry(diff).or_insert(price_sequence[i]);
        }
    }

    Buyer {
        final_secret,
        diff_to_count_map,
    }
}

fn p1(input: &str) -> usize {
    let initial_secrets = parse(input);
    initial_secrets
        .iter()
        .map(|&n| evolve(n, 2000).final_secret)
        .sum()
}

fn get_most_bananas(buyers: &[Buyer]) -> usize {
    let mut diff_to_total_count = HashMap::new();

    for diff_to_count in buyers.iter().map(|buyer| &buyer.diff_to_count_map) {
        for (diff, &count) in diff_to_count {
            *diff_to_total_count.entry(diff).or_insert(0) += count as usize;
        }
    }

    *diff_to_total_count.values().max().unwrap()
}

fn p2(input: &str) -> usize {
    let initial_secrets = parse(input);

    let buyers: Vec<Buyer> = initial_secrets.iter().map(|&n| evolve(n, 2000)).collect();

    get_most_bananas(&buyers)
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");

    #[test]
    fn test_p1_example1() {
        assert_eq!(p1(EXAMPLE1), 37327623);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(p2(EXAMPLE2), 23);
    }
}

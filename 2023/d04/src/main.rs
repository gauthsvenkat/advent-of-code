use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;

type Numbers = HashSet<usize>;
type Cards = BTreeMap<usize, (Numbers, Numbers)>;

fn parse(input: &str) -> Cards {
    let mut cards = BTreeMap::new();

    input.lines().for_each(|line| {
        if let Some((card_id, numbers)) = line.split_once(':') {
            let card_id: usize = card_id.replace("Card ", "").trim().parse().unwrap();

            if let Some((winning_numbers, our_numbers)) = numbers.split_once('|') {
                let winning_numbers: Numbers = winning_numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                let our_numbers: Numbers = our_numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                cards.insert(card_id, (winning_numbers, our_numbers));
            } else {
                panic!("Couldn't parse numbers");
            }
        } else {
            panic!("Couldn't parse line");
        }
    });

    cards
}

fn p1(input: &str) -> usize {
    let cards = parse(input);

    cards
        .iter()
        .map(
            |(_, (wn, on))| match on.iter().filter(|&n| wn.contains(n)).count() {
                0 => 0,
                mut n => {
                    let mut points = 0;
                    let mut i = 0;

                    while n - 1 > 0 {
                        points += 1 << i;

                        n -= 1;
                        i += 1;
                    }

                    points + 1
                }
            },
        )
        .sum()
}

fn p2(input: &str) -> usize {
    let cards = parse(input);
    let mut card_multiplier = BTreeMap::new();

    cards.iter().for_each(|(id, _)| {
        card_multiplier.insert(id, 1_usize);
    });

    cards.iter().for_each(|(id, (wn, on))| {
        let num_matches = on.iter().filter(|&n| wn.contains(n)).count();

        for cid in id + 1..=id + num_matches {
            *card_multiplier.get_mut(&cid).unwrap() += *card_multiplier.get(&id).unwrap();
        }
    });

    card_multiplier.values().sum()
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

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 13);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 30);
    }
}

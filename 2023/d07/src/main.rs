use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

struct CamelCard {
    hand: String,
    bid: usize,
}

fn card_to_score(c: &char, demote: bool) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => {
            if demote {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card"),
    }
}

impl CamelCard {
    fn hand_score(&self, promote: bool) -> usize {
        let mut counter = HashMap::new();

        self.hand
            .chars()
            .for_each(|c| *counter.entry(c).or_default() += 1);

        if promote {
            if let Some(j_count) = counter.remove(&'J') {
                if counter.is_empty() {
                    counter.insert('A', j_count);
                } else {
                    let max_count: &usize = counter.values().max().unwrap();
                    let promotion_candidates: Vec<&char> = counter
                        .iter()
                        .filter_map(|(k, &v)| if v == *max_count { Some(k) } else { None })
                        .collect();

                    let final_candidate: &char = match promotion_candidates.len() {
                        1 => promotion_candidates[0],
                        _ => promotion_candidates
                            .iter()
                            .max_by_key(|&&c| card_to_score(c, false))
                            .unwrap(),
                    };

                    counter
                        .entry(*final_candidate)
                        .and_modify(|v| *v += j_count);
                }
            }
        }

        let counts: Vec<usize> = {
            let mut counts: Vec<usize> = counter.values().cloned().collect();
            counts.sort_unstable();
            counts
        };

        match counter.len() {
            1 => 6,
            2 => match counts.as_slice() {
                [1, 4] => 5,
                [2, 3] => 4,
                _ => unreachable!(),
            },
            3 => match counts.as_slice() {
                [1, 1, 3] => 3,
                [1, 2, 2] => 2,
                _ => unreachable!(),
            },
            4 => 1,
            5 => 0,
            _ => unreachable!(),
        }
    }

    fn card_scores(&self, demote: bool) -> Vec<u8> {
        self.hand
            .chars()
            .map(|c| card_to_score(&c, demote))
            .collect()
    }
}

fn parse(input: &str) -> Vec<CamelCard> {
    input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();

            CamelCard {
                hand: l[0].to_string(),
                bid: l[1].parse().unwrap(),
            }
        })
        .collect()
}

fn sorter(a: &CamelCard, b: &CamelCard, promote: bool) -> Ordering {
    match a.hand_score(promote).cmp(&b.hand_score(promote)) {
        Ordering::Equal => {
            for (s1, s2) in a
                .card_scores(promote)
                .iter()
                .zip(b.card_scores(promote).iter())
            {
                match s1.cmp(s2) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            Ordering::Equal
        }
        other => other,
    }
}

fn p1(input: &str) -> usize {
    let mut camel_cards = parse(input);

    camel_cards.sort_by(|a, b| sorter(a, b, false));

    camel_cards
        .iter()
        .enumerate()
        .map(|(i, card)| card.bid * (i + 1))
        .sum()
}

fn p2(input: &str) -> usize {
    let mut camel_cards = parse(input);

    camel_cards.sort_by(|a, b| sorter(a, b, true));

    camel_cards
        .iter()
        .enumerate()
        .map(|(i, card)| card.bid * (i + 1))
        .sum()
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

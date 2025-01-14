use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use Card::*;
use Hand::*;

#[derive(Debug)]
struct CamelCard {
    hand: Vec<Card>,
    bid: usize,
}

impl CamelCard {
    fn hand_type(&self) -> Hand {
        let mut counter = HashMap::new();

        for c in self.hand.iter() {
            counter.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut counts: Vec<usize> = counter.values().cloned().collect();
        counts.sort_unstable();

        match counter.len() {
            1 => FiveOfAKind,
            2 => match counts.as_slice() {
                [1, 4] => FourOfAKind,
                [2, 3] => FullHouse,
                _ => unreachable!(),
            },
            3 => match counts.as_slice() {
                [1, 1, 3] => ThreeOfAKind,
                [1, 2, 2] => TwoPair,
                _ => unreachable!(),
            },
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other_card: &Self) -> Ordering {
        match self.hand_type().cmp(&other_card.hand_type()) {
            Ordering::Equal => {
                for (a, b) in self.hand.iter().zip(other_card.hand.iter()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CamelCard {}

fn parse(input: &str) -> Vec<CamelCard> {
    input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();

            CamelCard {
                hand: l[0]
                    .chars()
                    .map(|c| match c {
                        '2' => Two,
                        '3' => Three,
                        '4' => Four,
                        '5' => Five,
                        '6' => Six,
                        '7' => Seven,
                        '8' => Eight,
                        '9' => Nine,
                        'T' => Ten,
                        'J' => Jack,
                        'Q' => Queen,
                        'K' => King,
                        'A' => Ace,
                        _ => unreachable!(),
                    })
                    .collect(),
                bid: l[1].parse().unwrap(),
            }
        })
        .collect()
}

fn p1(input: &str) -> usize {
    let mut camel_cards = parse(input);
    camel_cards.sort_unstable();

    camel_cards
        .iter()
        .enumerate()
        .map(|(i, card)| card.bid * (i + 1))
        .sum()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    todo!()
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

use indexmap::IndexMap;
use std::{collections::BTreeMap, env, fs};

enum Operation {
    Equals(String, usize),
    Remove(String),
}

impl Operation {
    fn raw(&self) -> String {
        match self {
            Equals(l, fl) => format!("{}={}", l, fl),
            Remove(l) => format!("{}-", l),
        }
    }
}

use Operation::*;

fn parse(input: &str) -> Vec<Operation> {
    input
        .trim()
        .split_terminator(',')
        .map(|s| {
            if s.contains('=') {
                let (label, focal_length) = s.split_once('=').unwrap();

                Equals(label.to_string(), focal_length.parse().unwrap())
            } else {
                let (label, _) = s.split_once('-').unwrap();

                Remove(label.to_string())
            }
        })
        .collect()
}

fn hash(s: &str) -> usize {
    let mut val = 0;

    for &ac in s.as_bytes().iter() {
        val += ac as usize;
        val *= 17;
        val %= 256;
    }

    val
}

fn p1(input: &str) -> usize {
    parse(input).iter().map(|s| hash(&s.raw())).sum()
}

// label to focal length mapping, ordered by insertion order.
type Slots = IndexMap<String, usize>;
type Boxes = BTreeMap<usize, Slots>;

fn process(sequence: &[Operation]) -> Boxes {
    let mut boxes: Boxes = BTreeMap::new();

    for op in sequence {
        let box_id = match op {
            Equals(l, _) | Remove(l) => hash(l),
        };

        let slots = boxes.entry(box_id + 1).or_default();

        match op {
            Equals(l, fl) => {
                slots.insert(l.clone(), *fl);
            }
            Remove(l) => {
                slots.shift_remove(l);
            }
        }
    }

    boxes
}

fn p2(input: &str) -> usize {
    let sequence = parse(input);

    let boxes = process(&sequence);

    let mut focusing_power: usize = 0;

    for (box_id, slots) in boxes.iter() {
        for (i, (_, fl)) in slots.iter().enumerate() {
            focusing_power += box_id * (i + 1) * fl;
        }
    }

    focusing_power
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
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), 1320);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 145);
    }
}

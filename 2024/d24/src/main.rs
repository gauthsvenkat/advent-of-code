use regex::Regex;
use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::fs;

#[derive(Debug)]
enum Gate {
    And(String, String),
    Xor(String, String),
    Or(String, String),
}

fn parse(input: &str) -> (BTreeMap<String, bool>, BTreeMap<String, Gate>) {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut realized = BTreeMap::new();
    let re = Regex::new(r"([xy]\d{2}): (\d{1})").unwrap();

    groups[0].lines().for_each(|line| {
        let capture = re.captures(line).unwrap();

        let key = capture.get(1).unwrap().as_str().to_string();
        let value = capture
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .map(|x| x == 1)
            .unwrap();

        realized.insert(key, value);
    });

    let mut connections = BTreeMap::new();
    let re = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    groups[1].lines().for_each(|line| {
        let capture = re.captures(line).unwrap();

        let key = capture.get(4).unwrap().as_str().to_string();

        let a = capture.get(1).unwrap().as_str().to_string();
        let b = capture.get(3).unwrap().as_str().to_string();

        let gate = match capture.get(2).unwrap().as_str() {
            "AND" => Gate::And(a, b),
            "OR" => Gate::Or(a, b),
            "XOR" => Gate::Xor(a, b),
            _ => panic!("Invalid gate"),
        };

        connections.insert(key, gate);
    });

    (realized, connections)
}

fn eval_gates(
    mut realized: BTreeMap<String, bool>,
    connections: BTreeMap<String, Gate>,
) -> BTreeMap<String, bool> {
    let mut dq = connections.keys().collect::<VecDeque<_>>();

    while let Some(connection) = dq.pop_front() {
        match connections.get(connection).unwrap() {
            Gate::And(a, b) => {
                if let (Some(&va), Some(&vb)) = (realized.get(a), realized.get(b)) {
                    realized.insert(connection.to_string(), va & vb);
                } else {
                    dq.push_back(connection);
                }
            }
            Gate::Or(a, b) => {
                if let (Some(&va), Some(&vb)) = (realized.get(a), realized.get(b)) {
                    realized.insert(connection.to_string(), va | vb);
                } else {
                    dq.push_back(connection);
                }
            }
            Gate::Xor(a, b) => {
                if let (Some(&va), Some(&vb)) = (realized.get(a), realized.get(b)) {
                    realized.insert(connection.to_string(), va ^ vb);
                } else {
                    dq.push_back(connection);
                }
            }
        }
    }

    realized
}

fn p1(input: &str) -> usize {
    let (realized, connections) = parse(input);
    let realized = eval_gates(realized, connections);

    realized
        .iter()
        .filter_map(|(k, &v)| if k.starts_with('z') { Some(v) } else { None })
        .enumerate()
        .filter_map(|(i, bit)| if bit { Some(1 << i) } else { None })
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

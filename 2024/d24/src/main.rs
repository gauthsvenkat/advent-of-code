use regex::Regex;
use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::fs;

type Bits = BTreeMap<String, bool>;
type Gate = (String, String, String);
type Connections = BTreeMap<String, Gate>;
type InvertedConnections = BTreeMap<Gate, String>;

fn parse(input: &str) -> (Bits, Connections) {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut bits = BTreeMap::new();
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

        bits.insert(key, value);
    });

    let mut connections = BTreeMap::new();
    let re = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    groups[1].lines().for_each(|line| {
        let capture = re.captures(line).unwrap();

        let a = capture.get(1).unwrap().as_str().to_string();
        let op = capture.get(2).unwrap().as_str().to_string();
        let b = capture.get(3).unwrap().as_str().to_string();

        let key = capture.get(4).unwrap().as_str().to_string();

        connections.insert(key, (a, op, b));
    });

    (bits, connections)
}

fn eval(bits: &Bits, connections: &Connections) -> Bits {
    let mut dq: VecDeque<&String> = connections.keys().collect();
    let mut realized_bits = bits.clone();

    while let Some(conn) = dq.pop_front() {
        let (a, op, b) = connections.get(conn).unwrap();

        if let (Some(va), Some(vb)) = (realized_bits.get(a), realized_bits.get(b)) {
            realized_bits.insert(
                conn.to_string(),
                match op.as_str() {
                    "AND" => va & vb,
                    "OR" => va | vb,
                    "XOR" => va ^ vb,
                    _ => panic!("Invalid op"),
                },
            );
        } else {
            dq.push_back(conn);
        }
    }

    realized_bits
}

fn to_dec(bits: &Bits) -> usize {
    bits.iter()
        .filter_map(|(k, &v)| if k.starts_with('z') { Some(v) } else { None })
        .enumerate()
        .map(|(i, bit)| if bit { 1 << i } else { 0 })
        .sum()
}

fn p1(input: &str) -> usize {
    let (realized_bits, connections) = parse(input);
    let realized_bits = eval(&realized_bits, &connections);

    to_dec(&realized_bits)
}

fn get_bit(ic: &InvertedConnections, a: &str, b: &str, op: &str) -> String {
    if let Some(bit) = ic.get(&(a.to_string(), op.to_string(), b.to_string())) {
        bit.to_string()
    } else if let Some(bit) = ic.get(&(b.to_string(), op.to_string(), a.to_string())) {
        bit.to_string()
    } else {
        panic!("No {} gate found for {} and {}", op, a, b);
    }
}

fn analyze(ic: &InvertedConnections) {
    let mut carry = get_bit(ic, "x00", "y00", "AND");

    for i in 1..=44 {
        let x_gate = format!("x{:02}", i);
        let y_gate = format!("y{:02}", i);

        let s = get_bit(ic, &x_gate, &y_gate, "XOR");
        if s.starts_with('z') {
            println!("{} xor {} -> {}", x_gate, y_gate, s);
        }

        let z = get_bit(ic, &carry, &s, "XOR");
        if *z != format!("z{:02}", i) {
            println!("Expected z{:02}, got {}", i, z);
        }

        let p = get_bit(ic, &x_gate, &y_gate, "AND");
        if p.starts_with('z') {
            println!("{} and {} -> {}", x_gate, y_gate, p);
        }

        let q = get_bit(ic, &carry, &s, "AND");
        if q.starts_with('z') {
            println!("{} and {} -> {}", carry, s, q);
        }

        carry = get_bit(ic, &p, &q, "OR");
    }
}

fn p2(input: &str) -> usize {
    let (_, connections) = parse(input);

    let inverted_connections: InvertedConnections = connections
        .into_iter()
        .map(|(k, (a, op, b))| ((a, op, b), k))
        .collect();

    analyze(&inverted_connections);

    // Good luck writing code for this one
    // There wires are flipped
    // x12 AND y12 -> z12
    // pps XOR wdg -> vdc
    //
    // cdc OR stq -> z21
    // rsc XOR bbn -> nhn
    //
    // x25 XOR y25 -> khg
    // y25 AND x25 -> tvb
    //
    // jbr AND wcs -> z33
    // wcs XOR jbr -> gst

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

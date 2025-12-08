use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;

type Point = (usize, usize, usize);
type Circuit<'a> = BTreeSet<&'a Point>;
type Circuits<'a> = HashMap<usize, Circuit<'a>>;

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let pos: Vec<usize> = l.split(',').map(|n| n.parse::<usize>().unwrap()).collect();

            (pos[0], pos[1], pos[2])
        })
        .collect()
}

fn distance(p: &Point, q: &Point) -> f64 {
    // fn that is a proxy for euclidean distance. Taking the sqrt of the output would be
    // the euclidean distance.
    let d1 = (p.0 as f64 - q.0 as f64).powi(2);
    let d2 = (p.1 as f64 - q.1 as f64).powi(2);
    let d3 = (p.2 as f64 - q.2 as f64).powi(2);

    d1 + d2 + d3
}

fn get_closest_sorted_pairs(points: &[Point]) -> Vec<(&Point, &Point)> {
    // fn to take all possible pairs and sort them in ascending order of distance between them
    points
        .iter()
        .tuple_combinations() // take all possible pairs of points
        .map(|(p, q)| (p, q, distance(p, q))) // calculate the pair's distance as the 3rd element
        .sorted_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap()) // sort pair by distance
        .map(|(p, q, _)| (p, q)) // drop distance
        .collect()
}

fn init_circuits(points: &'_ [Point]) -> Circuits<'_> {
    // In the beginning of time, all points will be in their own circuits
    points
        .iter()
        .enumerate() // index will be used as the circuit_id
        .map(|(i, point)| (i, BTreeSet::from([point]))) // (circuit_id, `Circuit`)
        .collect() // collect as a HashMap where key = circuit_id and value is the `Circuit`
}

fn get_circuit_id(point: &Point, circuits: &Circuits) -> usize {
    for (&circuit_id, circuit) in circuits.iter() {
        if circuit.contains(point) {
            return circuit_id;
        }
    }

    unreachable!("Should have found circuit by now");
}

fn connect<'a>(pair: (&'a Point, &'a Point), mut circuits: Circuits<'a>) -> Circuits<'a> {
    // fn to take a pair of points and connect their respective circuits together
    // functionally, q's circuit is absorbed by p's circuit.
    let (p, q) = pair;

    let cid_p = get_circuit_id(p, &circuits);
    let cid_q = get_circuit_id(q, &circuits);

    if cid_p == cid_q {
        return circuits;
    }

    let q_circuit = circuits.remove(&cid_q).unwrap();
    let p_circuit = circuits.get_mut(&cid_p).unwrap();

    p_circuit.extend(q_circuit);

    circuits
}

fn p1(input: &str, n: usize) -> usize {
    let points: Vec<Point> = parse(input);

    let pairs = get_closest_sorted_pairs(&points).into_iter().take(n);

    let mut circuits = init_circuits(&points);

    for pair in pairs {
        circuits = connect(pair, circuits);
    }

    circuits
        .values()
        .map(|circuit| circuit.len())
        .sorted_unstable_by(|l1, l2| l1.cmp(l2).reverse())
        .take(3)
        .product()
}

fn p2(input: &str) -> usize {
    let points: Vec<Point> = parse(input);

    let pairs = get_closest_sorted_pairs(&points);

    let mut circuits = init_circuits(&points);

    let mut idx: Option<usize> = None;
    for (i, pair) in pairs.iter().enumerate() {
        circuits = connect(*pair, circuits);

        if circuits.len() == 1 {
            idx = Some(i);
            break;
        }
    }

    let (p, q) = pairs.get(idx.unwrap()).unwrap();

    p.0 * q.0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input, 1000)),
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
        assert_eq!(p1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 25272);
    }
}

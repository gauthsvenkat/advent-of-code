#[cfg_attr(test, allow(unused_imports))]
use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

type Outputs = HashSet<String>;
type Connections = HashMap<String, Outputs>;

fn parse(input: &str) -> Connections {
    input
        .lines()
        .map(|l| {
            let (device, outputs) = l.split_once(':').unwrap();
            let outputs: Outputs = outputs
                .strip_prefix(' ')
                .unwrap()
                .split(' ')
                .map(|o| o.to_owned())
                .collect();

            (device.to_owned(), outputs)
        })
        .collect()
}

// NOTE: The cache is global. Therefore, during testing the cache is polluted by the other part's
// solution, for both parts. So it is conditionally disabled for `test` compilation.
#[cfg_attr(
    not(test),
    cached(key = "String", convert = r#"{ format!("{}-{}", from, to) }"#)
)]
fn count_paths(from: &str, to: &str, connections: &Connections) -> usize {
    if from == to {
        return 1;
    }

    if from == "out" {
        return 0;
    }

    connections
        .get(from)
        .unwrap()
        .iter()
        .map(|d| count_paths(d, to, connections))
        .sum()
}

fn p1(input: &str) -> usize {
    let connections = parse(input);
    count_paths("you", "out", &connections)
}

fn p2(input: &str) -> usize {
    let connections = parse(input);

    let num_svr_fft_dac_out = count_paths("svr", "fft", &connections)
        * count_paths("fft", "dac", &connections)
        * count_paths("dac", "out", &connections);
    let num_svr_dac_fft_out = count_paths("svr", "dac", &connections)
        * count_paths("dac", "fft", &connections)
        * count_paths("fft", "out", &connections);

    // NOTE: Since the input is a DAG, one of these terms will be 0.
    // (it is the second one (dac -> fft) in my case)
    num_svr_fft_dac_out + num_svr_dac_fft_out
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE1), 5);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE2), 2);
    }
}

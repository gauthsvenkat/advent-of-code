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

fn count_you_to_out(current_device: &str, connections: &Connections) -> usize {
    if current_device == "out" {
        return 1;
    }

    connections
        .get(current_device)
        .unwrap()
        .iter()
        .map(|d| count_you_to_out(d, connections))
        .sum()
}

fn p1(input: &str) -> usize {
    let connections = parse(input);
    count_you_to_out("you", &connections)
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}-{}-{}", current_device, touched_dac, touched_fft) }"#
)]
fn count_svr_to_out(
    current_device: &str,
    connections: &Connections,
    touched_dac: bool,
    touched_fft: bool,
) -> usize {
    if current_device == "out" {
        return (touched_dac && touched_fft) as usize;
    }

    connections
        .get(current_device)
        .unwrap()
        .iter()
        .map(|d| {
            count_svr_to_out(
                d,
                connections,
                touched_dac || (current_device == "dac"),
                touched_fft || (current_device == "fft"),
            )
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let connections = parse(input);
    count_svr_to_out("svr", &connections, false, false)
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

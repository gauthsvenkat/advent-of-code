use std::collections::HashMap;
use std::env;
use std::fs;

type Map = HashMap<String, (String, String)>;

fn parse(input: &str) -> (String, Map) {
    let (instructions, map_raw) = if let Some((instructions, map)) = input.split_once("\n\n") {
        (
            instructions.trim().to_string(),
            map.trim().replace(['(', ')'], ""),
        )
    } else {
        panic!("Invalid input")
    };

    (
        instructions,
        map_raw
            .lines()
            .map(|l| {
                let (node, next_nodes) = l.split_once('=').unwrap();
                let (l, r) = next_nodes.split_once(',').unwrap();

                (
                    node.trim().to_string(),
                    (l.trim().to_string(), r.trim().to_string()),
                )
            })
            .collect(),
    )
}

fn num_step(start: &str, instructions: &str, map: &Map) -> usize {
    1 + instructions
        .chars()
        .cycle()
        .scan(start, |current, instruction| {
            *current = match instruction {
                'L' => &map.get(*current).unwrap().0,
                'R' => &map.get(*current).unwrap().1,
                _ => unreachable!(),
            };

            if current.ends_with('Z') {
                None
            } else {
                Some(*current)
            }
        })
        .count()
}

fn p1(input: &str) -> usize {
    let (instructions, map) = parse(input);

    num_step("AAA", &instructions, &map)
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

fn p2(input: &str) -> usize {
    let (instructions, map) = parse(input);

    map.keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| num_step(node, &instructions, &map))
        .fold(1, lcm)
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
    const EXAMPLE3: &str = include_str!("../eg3.txt");

    #[test]
    fn test_p1_example1() {
        assert_eq!(p1(EXAMPLE1), 2);
    }

    #[test]
    fn test_p1_example2() {
        assert_eq!(p1(EXAMPLE2), 6);
    }

    #[test]
    fn test_p2_example3() {
        assert_eq!(p2(EXAMPLE3), 6);
    }
}

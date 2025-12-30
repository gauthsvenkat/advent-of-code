use std::{env, fs};

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.split_whitespace().map(|n| n.parse().unwrap()))
        .collect()
}

fn count(input: &[usize]) -> usize {
    input
        .chunks(3)
        .filter(|chunk| {
            if let [a, b, c] = chunk {
                (a + b > *c) && (b + c > *a) && (a + c > *b)
            } else {
                false
            }
        })
        .count()
}

fn p1(input: &str) -> usize {
    count(&parse(input))
}

fn p2(input: &str) -> usize {
    let input = parse(input);

    let rearranged: Vec<usize> = input
        .iter()
        .step_by(3)
        .chain(input.iter().skip(1).step_by(3))
        .chain(input.iter().skip(2).step_by(3))
        .copied()
        .collect();

    count(&rearranged)
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
        assert_eq!(p1(EXAMPLE), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 6);
    }
}

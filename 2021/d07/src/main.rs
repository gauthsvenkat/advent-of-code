use std::{env, fs};

fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn p1(input: &str) -> usize {
    let positions = parse(input);

    let start = positions.iter().min().unwrap();
    let end = positions.iter().max().unwrap();

    (*start..=*end)
        .map(|p1| positions.iter().map(|p2| p2.abs_diff(p1)).sum())
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let positions = parse(input);

    let start = positions.iter().min().unwrap();
    let end = positions.iter().max().unwrap();

    (*start..=*end)
        .map(|p1| {
            positions
                .iter()
                .map(|p2| (0..=p2.abs_diff(p1)).sum::<usize>())
                .sum()
        })
        .min()
        .unwrap()
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
        assert_eq!(p1(EXAMPLE), 37);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 168);
    }
}

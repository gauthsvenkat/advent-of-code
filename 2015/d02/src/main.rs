use std::{env, fs};

type Dimension = (usize, usize, usize);

fn parse(input: &str) -> Vec<Dimension> {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<usize> = line.split('x').map(|n| n.parse().unwrap()).collect();
            dims.sort_unstable();

            (dims[0], dims[1], dims[2])
        })
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|(x, y, z)| 2 * (x * y + y * z + z * x) + x * y)
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|(x, y, z)| 2 * (x + y) + x * y * z)
        .sum()
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
        assert_eq!(p1(EXAMPLE), 101);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 48);
    }
}

use std::{env, fs, ops::Div};

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|m| m.div(3).saturating_sub(2))
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|m| {
            let mut t = *m;
            let mut acc: usize = 0;

            while t > 0 {
                let n = t.div(3).saturating_sub(2);
                acc += n;
                t = n;
            }

            acc
        })
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 34241);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 51316);
    }
}

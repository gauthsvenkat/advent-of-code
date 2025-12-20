use regex::Regex;
use std::env;
use std::fs;

type Button = (i64, i64);
type Prize = (i64, i64);
type Machine = (Button, Button, Prize);

fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let tuples: Vec<(i64, i64)> = re
                .captures_iter(block)
                .map(|cap| {
                    let x = cap[1].parse::<i64>().unwrap();
                    let y = cap[2].parse::<i64>().unwrap();
                    (x, y)
                })
                .collect();

            (tuples[0], tuples[1], tuples[2])
        })
        .collect()
}

fn solve(x: (i64, i64, i64), y: (i64, i64, i64)) -> Option<(i64, i64)> {
    let (a1, b1, c1) = x;
    let (a2, b2, c2) = y;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0 {
        return None;
    }

    let a_numerator = c1 * b2 - c2 * b1;
    let b_numerator = a1 * c2 - a2 * c1;

    if a_numerator % determinant == 0 && b_numerator % determinant == 0 {
        let a = a_numerator / determinant;
        let b = b_numerator / determinant;
        Some((a, b))
    } else {
        None
    }
}

fn p1(input: &str) -> i64 {
    let machines = parse(input);

    machines
        .iter()
        .map(|(button_a, button_b, prize)| {
            let (a, b) = solve(
                (button_a.0, button_b.0, prize.0),
                (button_a.1, button_b.1, prize.1),
            )
            .unwrap_or((0, 0));

            3 * a + b
        })
        .sum()
}

fn p2(input: &str) -> i64 {
    let machines = parse(input);

    machines
        .iter()
        .map(|(button_a, button_b, prize)| {
            let (a, b) = solve(
                (button_a.0, button_b.0, prize.0 + 10000000000000),
                (button_a.1, button_b.1, prize.1 + 10000000000000),
            )
            .unwrap_or((0, 0));

            3 * a + b
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
        assert_eq!(p1(EXAMPLE), 480);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 875318608908);
    }
}

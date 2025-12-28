use std::{env, fs};

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn predict(history: &Vec<isize>) -> (isize, isize) {
    let mut step = history.to_owned();

    let mut next = 0;
    let mut prev = 0;
    let mut sign = true;

    while !step.iter().all(|v| *v == 0) {
        next += step.last().unwrap();

        prev += step.first().unwrap() * if sign { 1 } else { -1 };
        sign = !sign;

        step = step.windows(2).map(|w| w[1] - w[0]).collect();
    }

    (prev, next)
}

fn p1(input: &str) -> isize {
    let histories = parse(input);

    histories.iter().map(predict).map(|(_, n)| n).sum()
}

fn p2(input: &str) -> isize {
    let histories = parse(input);

    histories.iter().map(predict).map(|(p, _)| p).sum()
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
        assert_eq!(p1(EXAMPLE), 114);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 2);
    }
}

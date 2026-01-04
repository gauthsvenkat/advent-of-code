use std::{collections::BTreeMap, env, fs};

fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn count(timers: &[usize], n_days: usize) -> usize {
    let mut counter = BTreeMap::new();

    timers.iter().for_each(|timer| {
        *counter.entry(*timer).or_default() += 1;
    });

    for _ in 0..n_days {
        let n_hatch = *counter.entry(0).or_default();
        counter.entry(0).and_modify(|v| *v = 0);

        for timer in 1..=8 {
            if let Some(val) = counter.remove(&timer) {
                counter.insert(timer - 1, val);
            }
        }

        *counter.entry(6).or_default() += n_hatch;
        *counter.entry(8).or_default() += n_hatch;
    }

    counter.values().sum()
}

fn p1(input: &str) -> usize {
    count(&parse(input), 80)
}

fn p2(input: &str) -> usize {
    count(&parse(input), 256)
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
        assert_eq!(p1(EXAMPLE), 5934);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 26984457539);
    }
}

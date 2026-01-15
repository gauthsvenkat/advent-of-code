use std::{
    env, fs,
    ops::{Add, Div, RangeInclusive},
};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn bsp(id: &str, range: RangeInclusive<usize>) -> usize {
    if id.is_empty() {
        return *range.start();
    }

    let l = *range.start();
    let r = *range.end();

    let m = r.add(l).div(2);

    let range = match id.chars().next().unwrap() {
        'F' | 'L' => l..=m,
        'B' | 'R' => (m + 1)..=r,
        c => panic!("Invalid character {c}"),
    };

    bsp(&id[1..], range)
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|id| {
            let row = bsp(&id[..7], 0..=127);
            let col = bsp(&id[7..], 0..=7);

            row * 8 + col
        })
        .max()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let mut seat_ids: Vec<_> = parse(input)
        .iter()
        .map(|id| {
            let row = bsp(&id[..7], 0..=127);
            let col = bsp(&id[7..], 0..=7);

            row * 8 + col
        })
        .collect();

    seat_ids.sort_unstable();

    seat_ids
        .windows(2)
        .find_map(|w| {
            if w[1].abs_diff(w[0]) > 1 {
                Some(w[0] + 1)
            } else {
                None
            }
        })
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
        assert_eq!(p1(EXAMPLE), 820);
    }

    #[test]
    fn test_p2() {
        // example input doesn't exactly fit the p2 criteria, so this is simply
        // a litmus test.
        assert_eq!(p2(EXAMPLE), 120);
    }
}

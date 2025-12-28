use std::{env, fs};

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| {
            let dir = l.chars().next().unwrap();
            let n: isize = l[1..].parse().unwrap();

            match dir {
                'L' => -n,
                'R' => n,
                _ => panic!("Invalid Direction"),
            }
        })
        .collect()
}

fn rotate(at: usize, rot: isize) -> usize {
    ((at as isize) + rot).rem_euclid(100) as usize
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);

    let mut password: usize = 0;
    let mut at: usize = 50;

    for rot in parsed_input.iter() {
        at = rotate(at, *rot);

        password += (at == 0) as usize;
    }

    password
}

fn num_pointing_at_zero(at: usize, rot: isize) -> usize {
    let offset = (at as isize) + rot;

    (offset.abs() / 100) as usize + (at != 0 && offset <= 0) as usize
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);

    let mut password: usize = 0;
    let mut at: usize = 50;

    for rot in parsed_input.iter() {
        password += num_pointing_at_zero(at, *rot);

        at = rotate(at, *rot);
    }

    password
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

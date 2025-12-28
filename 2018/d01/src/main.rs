use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

fn p1(input: &str) -> isize {
    parse(input).iter().sum()
}

fn p2(input: &str) -> isize {
    let changes = parse(input);

    let mut seen: HashSet<isize> = HashSet::new();
    let mut frequency: isize = 0;

    for delta in changes.iter().cycle() {
        seen.insert(frequency);

        frequency += delta;

        if seen.contains(&frequency) {
            return frequency;
        }
    }

    unreachable!("Get answer or loop forever")
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
    const EXAMPLE4: &str = include_str!("../eg4.txt");
    const EXAMPLE5: &str = include_str!("../eg5.txt");
    const EXAMPLE6: &str = include_str!("../eg6.txt");
    const EXAMPLE7: &str = include_str!("../eg7.txt");
    const EXAMPLE8: &str = include_str!("../eg8.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 3);
        assert_eq!(p1(EXAMPLE2), 3);
        assert_eq!(p1(EXAMPLE3), 0);
        assert_eq!(p1(EXAMPLE4), -6);
        assert_eq!(p1(EXAMPLE5), 0);
        assert_eq!(p1(EXAMPLE6), 4);
        assert_eq!(p1(EXAMPLE7), 4);
        assert_eq!(p1(EXAMPLE8), 1);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 2);
        assert_eq!(p2(EXAMPLE3), 0);
        assert_eq!(p2(EXAMPLE5), 0);
        assert_eq!(p2(EXAMPLE6), 10);
        assert_eq!(p2(EXAMPLE7), 5);
        assert_eq!(p2(EXAMPLE8), 14);
    }
}

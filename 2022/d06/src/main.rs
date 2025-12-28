use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn count_till_marker(buffer: &[char], n: usize) -> usize {
    buffer
        .windows(n)
        .enumerate()
        .find_map(|(i, sub)| {
            if sub.iter().collect::<HashSet<_>>().len() == n {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
        + n
}

fn p1(input: &str) -> usize {
    count_till_marker(&parse(input), 4)
}

fn p2(input: &str) -> usize {
    count_till_marker(&parse(input), 14)
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

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 7);
        assert_eq!(p1(EXAMPLE2), 5);
        assert_eq!(p1(EXAMPLE3), 6);
        assert_eq!(p1(EXAMPLE4), 10);
        assert_eq!(p1(EXAMPLE5), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 19);
        assert_eq!(p2(EXAMPLE2), 23);
        assert_eq!(p2(EXAMPLE3), 23);
        assert_eq!(p2(EXAMPLE4), 29);
        assert_eq!(p2(EXAMPLE5), 26);
    }
}

use std::{env, fs};

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn convert(c: &char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        c => panic!("Invalid instruction {c}"),
    }
}

fn p1(input: &str) -> i32 {
    parse(input).iter().map(convert).sum()
}

fn p2(input: &str) -> usize {
    let mut acc: i32 = 0;

    for (i, c) in parse(input).iter().enumerate() {
        acc += convert(c);

        if acc == -1 {
            return i + 1;
        }
    }

    unreachable!("Should have reached the basement by now")
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
    const EXAMPLE9: &str = include_str!("../eg9.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 0);
        assert_eq!(p1(EXAMPLE2), 0);
        assert_eq!(p1(EXAMPLE3), 3);
        assert_eq!(p1(EXAMPLE4), 3);
        assert_eq!(p1(EXAMPLE5), 3);
        assert_eq!(p1(EXAMPLE6), -1);
        assert_eq!(p1(EXAMPLE7), -1);
        assert_eq!(p1(EXAMPLE8), -3);
        assert_eq!(p1(EXAMPLE9), -3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE5), 1);
        assert_eq!(p2(EXAMPLE6), 3);
        assert_eq!(p2(EXAMPLE7), 1);
        assert_eq!(p2(EXAMPLE8), 1);
        assert_eq!(p2(EXAMPLE9), 1);
    }
}

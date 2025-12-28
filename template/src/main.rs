use std::{env, fs};

fn parse(input: &str) -> () {
    todo!()
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);
    todo!()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    todo!()
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
        // TODO:
        assert_eq!(p1(EXAMPLE), 0);
    }

    #[test]
    fn test_p2_example() {
        // TODO:
        assert_eq!(p2(EXAMPLE), 0);
    }
}

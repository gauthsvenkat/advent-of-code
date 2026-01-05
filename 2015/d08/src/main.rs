use std::{env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => continue,
            '\\' => match chars.next() {
                Some('x') => {
                    let hex: String = chars.by_ref().take(2).collect();

                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    }
                }
                Some(c) => result.push(c),
                None => {}
            },
            _ => result.push(ch),
        }
    }

    result
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|s| s.len() - decode(s).chars().count())
        .sum()
}

fn encode(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push('"');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            _ => result.push(c),
        }
    }

    format!("\"{result}\"")
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|s| encode(s).chars().count() - s.len())
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
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), 12);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 19);
    }
}

use std::{env, fs};

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks: Vec<Vec<u8>> = Vec::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();

    input.split("\n\n").for_each(|b| {
        let block = b.split("\n").collect::<Vec<&str>>();

        let mut col_heights: Vec<u8> = Vec::new();

        let num_columns = block[0].len();

        for j in 0..num_columns {
            let mut col_height = 0;
            for row in &block {
                if let Some(c) = row.chars().nth(j) {
                    if c == '#' {
                        col_height += 1;
                    }
                }
            }
            col_heights.push(col_height - 1);
        }

        if block[0].starts_with("#####") {
            locks.push(col_heights);
        } else {
            keys.push(col_heights);
        }
    });

    (locks, keys)
}

fn p1(input: &str) -> usize {
    let (locks, keys) = parse(input);

    let mut fit_count = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key).all(|(l, k)| l + k < 6) {
                fit_count += 1;
            }
        }
    }

    fit_count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 3);
    }
}

use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn parse_row(row: &[String]) -> Vec<usize> {
    row.iter().map(|n| n.parse::<usize>().unwrap()).collect()
}

#[allow(clippy::ptr_arg)]
fn transpose<T: Clone + Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n_col = matrix[0].len();

    let mut t_matrix = vec![vec![]; n_col];

    for row in matrix.iter() {
        for j in 0..row.len() {
            t_matrix[j].push(row[j]);
        }
    }

    t_matrix
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);

    let mut parsed_input: Vec<Vec<String>> = parsed_input
        .iter()
        .map(|row| {
            row.iter()
                .collect::<String>()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect()
        })
        .collect();

    let sign_row = parsed_input.pop().unwrap();

    let worksheet = transpose(&parsed_input.iter().map(|row| parse_row(row)).collect());

    worksheet
        .iter()
        .zip(sign_row)
        .map(|(row, sign)| {
            if sign == "*" {
                row.iter().product::<usize>()
            } else {
                row.iter().sum()
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    let matrix = transpose(&parsed_input);

    let mut answer: usize = 0;

    // buffer var to hold entire problems (columns in the input)
    let mut problem: Vec<usize> = vec![];
    // buffer to hold the problem's sign
    let mut sign_buffer: Option<char> = None;

    for (i, row) in matrix.iter().enumerate() {
        // buffer to continually parse for digits
        let mut num_buffer = String::new();

        for &c in row {
            if c == ' ' {
                continue;
            } else if c == '*' || c == '+' {
                sign_buffer = Some(c);
            } else {
                num_buffer.push(c);
            }
        }

        if !num_buffer.is_empty() {
            // parse and add the digit to the problem
            problem.push(num_buffer.parse().unwrap());
        }

        // if the row is empty or is the last one, we should have all the
        // right numbers in `problem` and the sign should be in `sign_buffer`
        if row.iter().all(|c| c.is_whitespace()) || (i == matrix.len() - 1) {
            answer += if sign_buffer.unwrap() == '*' {
                problem.iter().product::<usize>()
            } else {
                problem.iter().sum()
            };

            // reset to parse next problem
            problem = vec![];
            sign_buffer = None;
        }
    }

    answer
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
        assert_eq!(p1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 3263827);
    }
}

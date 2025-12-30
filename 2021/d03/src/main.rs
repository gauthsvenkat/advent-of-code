use std::{env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

struct Counter {
    n_ones: usize,
    n_zeroes: usize,
}

fn count(report: &[String], col_idx: usize) -> Counter {
    let n_row = report.len();

    let n_ones = report
        .iter()
        .filter(|s| s.chars().nth(col_idx).unwrap() == '1')
        .count();

    Counter {
        n_ones,
        n_zeroes: n_ones.abs_diff(n_row),
    }
}

fn p1(input: &str) -> usize {
    let report = parse(input);

    let n_col = report[0].len();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..n_col {
        let Counter { n_ones, n_zeroes } = count(&report, i);

        if n_ones > n_zeroes {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn filter(report: &[String], col_idx: usize, pref: char) -> Vec<String> {
    let Counter { n_ones, n_zeroes } = count(report, col_idx);

    let most_common_char = if n_ones > n_zeroes {
        '1'
    } else if n_zeroes > n_ones {
        '0'
    } else {
        pref
    };

    let least_common_char = if n_ones < n_zeroes {
        '1'
    } else if n_zeroes < n_ones {
        '0'
    } else {
        pref
    };

    let c = match pref {
        '1' => most_common_char,
        '0' => least_common_char,
        c => panic!("Invalid digit {c}!"),
    };

    report
        .iter()
        .filter(|s| s.chars().nth(col_idx).unwrap() == c)
        .map(|s| s.to_string())
        .collect()
}

fn filter_to_one(report: &[String], pref: char) -> String {
    let n_col = report[0].len();
    let mut under_consideration: Vec<String> = report.iter().map(|s| s.to_owned()).collect();

    for i in 0..n_col {
        under_consideration = filter(&under_consideration, i, pref);

        if under_consideration.len() == 1 {
            break;
        }
    }

    under_consideration.into_iter().next().unwrap()
}

fn p2(input: &str) -> usize {
    let report = parse(input);

    let oxygen_generator_rating = filter_to_one(&report, '1');
    let co2_scrubber_rating = filter_to_one(&report, '0');

    let oxygen_generator_rating = usize::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
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
        assert_eq!(p1(EXAMPLE), 198);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 230);
    }
}

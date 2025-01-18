use std::env;
use std::fs;

type Pattern = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut transposed = vec![vec![' '; pattern.len()]; pattern[0].len()];

    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            transposed[j][i] = pattern[i][j];
        }
    }

    transposed
}

fn count_disagreements(first_half: &[Vec<char>], second_half: &[Vec<char>]) -> usize {
    let mut count = 0;

    for (row_first, row_second) in first_half.iter().rev().zip(second_half.iter()) {
        for (a, b) in row_first.iter().zip(row_second.iter()) {
            if a != b {
                count += 1;
            }
        }
    }

    count
}

fn find_reflection(pattern: &Pattern, smudge: bool) -> Option<usize> {
    for i in 1..pattern.len() {
        let (first_half, second_half) = pattern.split_at(i);

        match count_disagreements(first_half, second_half) {
            0 if !smudge => return Some(i),
            1 if smudge => return Some(i),
            _ => continue,
        }
    }

    None
}

fn p1(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .iter()
        .map(|pattern| {
            if let Some(i) = find_reflection(pattern, false) {
                100 * i
            } else if let Some(i) = find_reflection(&transpose(pattern), false) {
                i
            } else {
                panic!("Invalid pattern");
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .iter()
        .map(|pattern| {
            if let Some(i) = find_reflection(pattern, true) {
                100 * i
            } else if let Some(i) = find_reflection(&transpose(pattern), true) {
                i
            } else {
                panic!("Invalid pattern");
            }
        })
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

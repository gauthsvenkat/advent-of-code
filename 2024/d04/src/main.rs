use std::{env, fs};

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn diagonal_lr(grid: &[Vec<char>]) -> Vec<String> {
    let grid_length: usize = grid.len();

    let mut diagonal_strings: Vec<String> = Vec::new();

    for c in 0..(grid_length - 3) {
        // print!("Starting at ({}, {}) -> ", 0, c);
        let mut diagonal_string: String = String::new();

        for (i, j) in (0..grid_length).zip(c..grid_length) {
            // print!("({}, {}) ", i, j);
            diagonal_string.push(grid[i][j]);
        }
        diagonal_strings.push(diagonal_string)
        // println!("");
    }

    for r in 1..(grid_length - 3) {
        // print!("Starting at ({}, {}) -> ", r, 0);
        let mut diagonal_string: String = String::new();

        for (i, j) in (r..grid_length).zip(0..grid_length) {
            // print!("({}, {}) ", i, j);
            diagonal_string.push(grid[i][j]);
        }
        diagonal_strings.push(diagonal_string)
        // println!("");
    }

    diagonal_strings
}

fn diagonal_rl(grid: &[Vec<char>]) -> Vec<String> {
    let grid_length: usize = grid.len();

    let mut diagonal_strings: Vec<String> = Vec::new();

    for c in (3..grid_length).rev() {
        // print!("Starting at ({}, {}) -> ", 0, c);
        let mut diagonal_string: String = String::new();
        for (i, j) in (0..grid_length).zip((0..c + 1).rev()) {
            // print!("({}, {}) ", i, j);
            diagonal_string.push(grid[i][j]);
        }
        diagonal_strings.push(diagonal_string)
        // println!("");
    }

    for r in 1..(grid_length - 3) {
        // print!("Starting at ({}, {}) -> ", r, grid_length - 1);
        let mut diagonal_string: String = String::new();
        for (i, j) in (r..grid_length).zip((0..grid_length).rev()) {
            // print!("({}, {}) ", i, j);
            diagonal_string.push(grid[i][j]);
        }
        diagonal_strings.push(diagonal_string)
        // println!("");
    }

    diagonal_strings
}

fn top_down(grid: &[Vec<char>]) -> Vec<String> {
    let grid_length: usize = grid.len();

    let mut top_down_strings: Vec<String> = Vec::new();

    for c in 0..grid_length {
        let mut top_down_string: String = String::new();
        for row in grid.iter() {
            top_down_string.push(row[c]);
        }
        top_down_strings.push(top_down_string);
    }

    top_down_strings
}

fn left_right(grid: &[Vec<char>]) -> Vec<String> {
    let mut left_right_strings: Vec<String> = Vec::new();

    for row in grid.iter() {
        let left_right_string: String = row.iter().collect();
        left_right_strings.push(left_right_string);
    }

    left_right_strings
}

fn count(strings: &[String]) -> usize {
    let mut count: usize = 0;
    for string in strings.iter() {
        count += string.matches("XMAS").count();
        count += string.matches("SAMX").count();
    }
    count
}

fn p1(input: &str) -> usize {
    let reports = parse(input);

    let dlr = diagonal_lr(&reports);
    let drl = diagonal_rl(&reports);
    let td = top_down(&reports);
    let lr = left_right(&reports);

    count(&dlr) + count(&drl) + count(&td) + count(&lr)
}

fn threexthree_window(grid: &[Vec<char>]) -> Vec<String> {
    let grid_length: usize = grid.len();
    let mut threexthree_strings: Vec<String> = Vec::new();
    for r in 0..(grid_length - 2) {
        for c in 0..(grid_length - 2) {
            let mut threexthree_string: String = String::new();
            for row in grid.iter().skip(r).take(3) {
                for &ch in row.iter().skip(c).take(3) {
                    threexthree_string.push(ch);
                }
            }
            threexthree_strings.push(threexthree_string);
        }
    }
    threexthree_strings
}

fn count_mas(strings: &[String]) -> usize {
    let mut count: usize = 0;

    for txt in strings.iter() {
        let txt_chars: Vec<char> = txt.chars().collect();

        if ((txt_chars[0] == 'M' && txt_chars[4] == 'A' && txt_chars[8] == 'S')
            || (txt_chars[0] == 'S' && txt_chars[4] == 'A' && txt_chars[8] == 'M'))
            && ((txt_chars[2] == 'M' && txt_chars[4] == 'A' && txt_chars[6] == 'S')
                || (txt_chars[2] == 'S' && txt_chars[4] == 'A' && txt_chars[6] == 'M'))
        {
            count += 1;
        }
    }

    count
}

fn p2(input: &str) -> usize {
    let reports = parse(input);

    let txt = threexthree_window(&reports);

    count_mas(&txt)
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
        assert_eq!(p1(EXAMPLE), 18);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 9);
    }
}

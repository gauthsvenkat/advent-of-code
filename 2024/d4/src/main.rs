use std::env;
use std::fs;

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path).unwrap()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn diagonal_lr(grid: &Vec<Vec<char>>) -> Vec<String> {
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

fn diagonal_rl(grid: &Vec<Vec<char>>) -> Vec<String> {
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

fn p1(input: &str) -> u32 {
    let reports = parse(input);
    let diagonal_strings = diagonal_rl(&reports);
    dbg!(diagonal_strings);
    0
}

fn p2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = get_input();

    let p1_sol = p1(&input);
    println!("{p1_sol}");

    // let p2_sol = p2(&input);
    // println!("{p2_sol}");
}

use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn has_neighboring_symbol(grid: &[Vec<char>], dims: (usize, usize), pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let (n_rows, n_cols) = dims;

    for (di, dj) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (ni, nj) = (i as isize + di, j as isize + dj);

        if ni < 0 || nj < 0 || ni >= (n_rows as isize) || nj >= (n_cols as isize) {
            continue;
        }

        let (ni, nj) = (ni as usize, nj as usize);

        if grid[ni][nj] != '.' && grid[ni][nj].is_ascii_punctuation() {
            return true;
        }
    }

    false
}

fn process(grid: &[Vec<char>]) -> usize {
    let (n_rows, n_cols) = (grid.len(), grid[0].len());

    let mut total = 0;

    let mut buffer = String::new();
    let mut is_adjacent = false;

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                buffer.push(c);

                is_adjacent = is_adjacent || has_neighboring_symbol(grid, (n_rows, n_cols), (i, j));
            }

            if (!c.is_ascii_digit() || j == n_cols - 1) && !buffer.is_empty() {
                if is_adjacent {
                    let num = buffer.parse::<usize>().unwrap();
                    dbg!(num);
                    total += num;
                }

                buffer.clear();
                is_adjacent = false;
            }
        }
    }

    total
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    process(&grid)
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

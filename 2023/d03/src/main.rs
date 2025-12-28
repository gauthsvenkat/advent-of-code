use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn has_neighboring_symbol(
    grid: &[Vec<char>],
    dims: (usize, usize),
    pos: (usize, usize),
    symbol: Option<char>,
) -> Option<(usize, usize)> {
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

        if grid[ni][nj] != '.' {
            if let Some(symbol) = symbol {
                if grid[ni][nj] == symbol {
                    return Some((ni, nj));
                }
            } else if grid[ni][nj].is_ascii_punctuation() {
                return Some((ni, nj));
            }
        }
    }

    None
}

fn process(grid: &[Vec<char>]) -> (usize, usize) {
    let (n_rows, n_cols) = (grid.len(), grid[0].len());

    let mut total = 0;
    let mut gear_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let mut buffer = String::new();
    let mut is_adjacent = false;
    let mut gear_location: Option<(usize, usize)> = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                buffer.push(c);

                is_adjacent = is_adjacent
                    || has_neighboring_symbol(grid, (n_rows, n_cols), (i, j), None).is_some();

                gear_location = if gear_location.is_some() {
                    gear_location
                } else {
                    has_neighboring_symbol(grid, (n_rows, n_cols), (i, j), Some('*'))
                }
            }

            if (!c.is_ascii_digit() || j == n_cols - 1) && !buffer.is_empty() {
                if is_adjacent {
                    let num = buffer.parse::<usize>().unwrap();
                    total += num;

                    if let Some(gear_location) = gear_location {
                        gear_map.entry(gear_location).or_default().push(num);
                    }
                }

                buffer.clear();
                is_adjacent = false;
                gear_location = None;
            }
        }
    }

    let gear_ratio = gear_map
        .into_iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<usize>())
        .sum();

    (total, gear_ratio)
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    process(&grid).0
}

fn p2(input: &str) -> usize {
    let grid = parse(input);
    process(&grid).1
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
        assert_eq!(p1(EXAMPLE), 4361);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 467835);
    }
}

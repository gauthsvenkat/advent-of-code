use std::collections::HashMap;
use std::env;
use std::fs;

type Point = (usize, usize);
type Grid = HashMap<Point, u8>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars().enumerate().map(move |(j, ch)| {
                let height: u8 = ch.to_digit(10).unwrap() as u8;

                ((i, j), height)
            })
        })
        .collect()
}

fn is_visible(point: &Point, grid: &Grid) -> bool {
    let max_row_idx = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_col_idx = grid.keys().map(|(_, y)| y).max().unwrap();

    // Check if on edge
    if point.0 == 0 || point.0 == *max_row_idx || point.1 == 0 || point.1 == *max_col_idx {
        return true;
    }

    let (x, y) = point;
    let height = grid.get(point).unwrap();

    // visible from right?
    if (y + 1..=*max_col_idx).all(|j| grid.get(&(*x, j)).unwrap() < height) {
        return true;
    }

    // visible from left?
    if (0..*y).all(|j| grid.get(&(*x, j)).unwrap() < height) {
        return true;
    }

    // visible from bottom?
    if (x + 1..=*max_row_idx).all(|i| grid.get(&(i, *y)).unwrap() < height) {
        return true;
    }

    // visible from top?
    if (0..*x).all(|i| grid.get(&(i, *y)).unwrap() < height) {
        return true;
    }

    false
}

fn p1(input: &str) -> usize {
    let grid = parse(input);

    grid.keys().filter(|p| is_visible(p, &grid)).count()
}

fn viewing_distance(heights: &[&u8], height: &u8) -> usize {
    let mut d = 0;

    for h in heights {
        d += 1;
        if *h >= height {
            break;
        }
    }

    d
}

fn scenic_score(point: &Point, grid: &Grid) -> usize {
    let max_row_idx = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_col_idx = grid.keys().map(|(_, y)| y).max().unwrap();

    let (x, y) = point;
    let height = grid.get(point).unwrap();

    let right: Vec<_> = (y + 1..=*max_col_idx)
        .map(|j| grid.get(&(*x, j)).unwrap())
        .collect();

    let left: Vec<_> = (0..*y).rev().map(|j| grid.get(&(*x, j)).unwrap()).collect();

    let bottom: Vec<_> = (x + 1..=*max_row_idx)
        .map(|i| grid.get(&(i, *y)).unwrap())
        .collect();

    let top: Vec<_> = (0..*x).rev().map(|i| grid.get(&(i, *y)).unwrap()).collect();

    viewing_distance(&right, height)
        * viewing_distance(&left, height)
        * viewing_distance(&bottom, height)
        * viewing_distance(&top, height)
}

fn p2(input: &str) -> usize {
    let grid = parse(input);

    grid.keys().map(|p| scenic_score(p, &grid)).max().unwrap()
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
        assert_eq!(p1(EXAMPLE), 21);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 8);
    }
}

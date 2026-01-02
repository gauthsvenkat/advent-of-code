use std::{collections::HashMap, env, fs};

type Point = (isize, isize);

struct Grid {
    base: HashMap<Point, char>,
    n_cols: isize,
}

fn parse(input: &str) -> Grid {
    let base = input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as isize, j as isize), ch))
        })
        .collect();

    let n_cols = input
        .lines()
        .next()
        .map(|line| line.chars().count())
        .unwrap();

    Grid {
        base,
        n_cols: n_cols as isize,
    }
}

impl Grid {
    fn get(&self, pos: &Point) -> Option<&char> {
        let (x, y) = pos;
        self.base.get(&(*x, y % self.n_cols))
    }
}

fn traverse(grid: &Grid, slope: &Point) -> usize {
    let mut pos = (0, 0);
    let mut count = 0;

    while let Some(ch) = grid.get(&pos) {
        if *ch == '#' {
            count += 1;
        }

        let (x, y) = pos;
        let (dx, dy) = slope;

        pos = (x + dx, y + dy);
    }

    count
}

fn p1(input: &str) -> usize {
    traverse(&parse(input), &(1, 3))
}

fn p2(input: &str) -> usize {
    let grid = parse(input);

    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|slope| traverse(&grid, slope))
        .product()
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
        assert_eq!(p1(EXAMPLE), 7);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 336);
    }
}

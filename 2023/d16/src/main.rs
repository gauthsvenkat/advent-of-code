use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

struct Map {
    grid: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

type Position = (usize, usize);
type Visited = HashMap<Position, HashSet<char>>;

fn parse(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    Map {
        n_rows: grid.len(),
        n_cols: grid[0].len(),
        grid,
    }
}

fn travel(map: &Map, vector: (usize, usize, char)) -> usize {
    let mut visited: Visited = HashMap::new();
    let mut to_visit = vec![vector];

    while let Some((i, j, d)) = to_visit.pop() {
        if let Some(dirs) = visited.get_mut(&(i, j)) {
            if dirs.contains(&d) {
                continue;
            } else {
                dirs.insert(d);
            }
        } else {
            visited.insert((i, j), HashSet::from([d]));
        }

        let c = map.grid[i][j];

        match (c, d) {
            ('.' | '-', '<') if j > 0 => to_visit.push((i, j - 1, d)),
            ('.' | '|', 'v') if i < map.n_rows - 1 => to_visit.push((i + 1, j, d)),
            ('.' | '|', '^') if i > 0 => to_visit.push((i - 1, j, d)),
            ('.' | '-', '>') if j < map.n_cols - 1 => to_visit.push((i, j + 1, d)),
            ('\\', '<') | ('/', '>') if i > 0 => to_visit.push((i - 1, j, '^')),
            ('\\', 'v') | ('/', '^') if j < map.n_cols - 1 => to_visit.push((i, j + 1, '>')),
            ('\\', '^') | ('/', 'v') if j > 0 => to_visit.push((i, j - 1, '<')),
            ('\\', '>') | ('/', '<') if i < map.n_rows - 1 => to_visit.push((i + 1, j, 'v')),
            ('|', '<' | '>') => {
                if i > 0 {
                    to_visit.push((i - 1, j, '^'));
                }
                if i < map.n_rows - 1 {
                    to_visit.push((i + 1, j, 'v'));
                }
            }
            ('-', '^' | 'v') => {
                if j > 0 {
                    to_visit.push((i, j - 1, '<'));
                }
                if j < map.n_cols - 1 {
                    to_visit.push((i, j + 1, '>'));
                }
            }
            _ => continue,
        }
    }

    visited.len()
}

fn p1(input: &str) -> usize {
    let map = parse(input);

    travel(&map, (0, 0, '>'))
}

fn p2(input: &str) -> usize {
    let map = parse(input);

    let mut result = 0;

    for r in 0..map.n_rows {
        for (c, d) in [(0, '>'), (map.n_cols - 1, '<')] {
            result = result.max(travel(&map, (r, c, d)));
        }
    }

    for c in 0..map.n_cols {
        for (r, d) in [(0, 'v'), (map.n_rows - 1, '^')] {
            result = result.max(travel(&map, (r, c, d)));
        }
    }

    result
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

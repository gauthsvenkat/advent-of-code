use std::{collections::HashMap, env, fs};

type Position = (usize, usize);

struct Map {
    grid: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

fn parse(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    Map {
        n_rows: grid.len(),
        n_cols: grid[0].len(),
        grid,
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

use Direction::*;

fn step(map: &mut Map, (i, j): Position, direction: &Direction) -> Option<Position> {
    let c = map.grid[i][j];

    let (ni, nj) = match direction {
        North => {
            if i == 0 {
                return None;
            }

            (i - 1, j)
        }
        West => {
            if j == 0 {
                return None;
            }
            (i, j - 1)
        }
        South => {
            if i == map.n_rows - 1 {
                return None;
            }

            (i + 1, j)
        }
        East => {
            if j == map.n_cols - 1 {
                return None;
            }
            (i, j + 1)
        }
    };

    match map.grid[ni][nj] {
        '.' => {
            map.grid[ni][nj] = c;
            map.grid[i][j] = '.';

            Some((ni, nj))
        }
        'O' => {
            if step(map, (ni, nj), direction).is_some() {
                step(map, (i, j), direction)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn tilt(map: &mut Map, pos: Position, direction: &Direction) -> Position {
    let mut pos = pos;

    while let Some(next_pos) = step(map, pos, direction) {
        pos = next_pos;
    }

    pos
}

fn get_load(map: &Map) -> usize {
    let mut load = 0;

    for i in 0..map.n_rows {
        for j in 0..map.n_cols {
            if map.grid[i][j] == 'O' {
                load += map.n_rows - i;
            }
        }
    }

    load
}

fn p1(input: &str) -> usize {
    let mut map = parse(input);

    for i in 0..map.n_rows {
        for j in 0..map.n_cols {
            if map.grid[i][j] == 'O' {
                let _ = tilt(&mut map, (i, j), &North);
            }
        }
    }

    get_load(&map)
}

fn cycle(map: &mut Map) {
    for dir in [North, West, South, East] {
        for i in 0..map.n_rows {
            for j in 0..map.n_cols {
                if map.grid[i][j] == 'O' {
                    let _ = tilt(map, (i, j), &dir);
                }
            }
        }
    }
}

fn p2(input: &str) -> usize {
    let mut map = parse(input);

    let mut i = 0;
    let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    while !seen.contains_key(&map.grid) {
        seen.insert(map.grid.clone(), i);
        cycle(&mut map);
        i += 1;
    }

    let cycle_start = seen.get(&map.grid).unwrap();
    let cycle_end = i;
    let final_grid = (1000000000 - cycle_start) % (cycle_end - cycle_start) + cycle_start;

    map.grid = seen
        .iter()
        .find_map(|(key, &val)| {
            if val == final_grid {
                Some(key.clone())
            } else {
                None
            }
        })
        .unwrap();

    get_load(&map)
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
        assert_eq!(p1(EXAMPLE), 136);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 64);
    }
}

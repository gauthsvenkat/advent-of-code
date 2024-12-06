use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn readfile(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn get_pos_and_dir(grid: &[Vec<char>]) -> ((usize, usize), char) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if matches!(cell, '<' | 'v' | '^' | '>') {
                return ((i, j), cell);
            }
        }
    }

    panic!("No starting position found");
}

fn step(grid: &[Vec<char>], pos: (usize, usize), dir: char) -> ((usize, usize), char) {
    let (x, y) = pos;

    match dir {
        '<' => {
            if grid[x][y - 1] != '#' {
                ((x, y - 1), dir)
            } else {
                step(grid, pos, '^')
            }
        }
        'v' => {
            if grid[x + 1][y] != '#' {
                ((x + 1, y), dir)
            } else {
                step(grid, pos, '<')
            }
        }
        '^' => {
            if grid[x - 1][y] != '#' {
                ((x - 1, y), dir)
            } else {
                step(grid, pos, '>')
            }
        }
        '>' => {
            if grid[x][y + 1] != '#' {
                ((x, y + 1), dir)
            } else {
                step(grid, pos, 'v')
            }
        }
        _ => panic!("Invalid direction"),
    }
}

fn travel(
    grid: &[Vec<char>],
    pos: (usize, usize),
    dir: char,
    mut record: HashMap<(usize, usize), HashSet<char>>,
) -> (HashMap<(usize, usize), HashSet<char>>, bool) {
    let (x, y) = pos;

    if let Some(dirs) = record.get(&pos) {
        if dirs.contains(&dir) {
            return (record, true);
        }
    }

    record.entry(pos).or_default().insert(dir);

    if (dir == '<' && y == 0)
        || (dir == 'v' && x == grid.len() - 1)
        || (dir == '^' && x == 0)
        || (dir == '>' && y == grid[x].len() - 1)
    {
        return (record, false);
    }

    let (new_pos, new_dir) = step(grid, pos, dir);

    travel(grid, new_pos, new_dir, record)
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    let (pos, dir) = get_pos_and_dir(&grid);

    let (record, _) = travel(&grid, pos, dir, HashMap::new());

    record.len()
}

fn p2(input: &str) -> usize {
    let mut grid = parse(input);
    let (starting_pos, dir) = get_pos_and_dir(&grid);

    let (mut record, _) = travel(&grid, starting_pos, dir, HashMap::new());

    // Remove start position
    record.remove(&starting_pos);

    // Insert a # at each visited position
    // and check if there is a loop
    record
        .keys()
        .filter(|&pos| {
            let (x, y) = pos;
            grid[*x][*y] = '#';
            let (_, has_loop) = travel(&grid, starting_pos, dir, HashMap::new());
            grid[*x][*y] = '.';
            has_loop
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = readfile(filepath);

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input)),
        _ => panic!("Invalid part"),
    };
}

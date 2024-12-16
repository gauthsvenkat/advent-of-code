use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_start(maze: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No start found");
}

fn solver(
    maze: &[Vec<char>],
    posd: ((usize, usize), char),
    mut visited: HashSet<(usize, usize)>,
) -> Option<usize> {
    let (position, direction) = posd;
    let (i, j) = position;

    if maze[i][j] == 'E' {
        return Some(0);
    } else if visited.contains(&position) {
        return None;
    }

    visited.insert(position);

    let next_directions = match direction {
        '<' => ['<', 'v', '^'],
        'v' => ['v', '>', '<'],
        '^' => ['^', '<', '>'],
        '>' => ['>', '^', 'v'],
        _ => panic!("Invalid direction"),
    };
    // NOTE: Check if it makes sense to turn and move in one step.
    let penalties = [1, 1001, 1001];

    // TODO: Use these directions by manhatten distance or something
    let mut next_scores: Vec<usize> = Vec::new();

    for (dir, penalty) in next_directions.into_iter().zip(penalties) {
        let dir_vec = match dir {
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            '>' => (0, 1),
            _ => panic!("Invalid direction"),
        };

        let next_pos = (
            (i as i32 + dir_vec.0) as usize,
            (j as i32 + dir_vec.1) as usize,
        );

        if maze[next_pos.0][next_pos.1] == '#' {
            continue;
        } else if let Some(next_score) = solver(maze, (next_pos, dir), visited.clone()) {
            next_scores.push(penalty + next_score);
        } else {
            continue;
        }
    }

    next_scores.into_iter().min()
}

fn p1(input: &str) -> usize {
    let maze = parse(input);
    let start = find_start(&maze);

    solver(&maze, (start, '>'), HashSet::new()).expect("No solution found!")
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

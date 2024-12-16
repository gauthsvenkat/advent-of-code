use std::collections::HashSet;
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

fn step(
    maze: &[Vec<char>],
    posd: ((usize, usize), char),
    curr_score: usize,
    mut best_score: usize,
    mut visited: HashSet<(usize, usize)>,
) -> (usize, usize) {
    let ((i, j), dir) = posd;

    if curr_score >= best_score || visited.contains(&(i, j)) {
        return (usize::MAX, best_score);
    }

    visited.insert((i, j));

    if maze[i][j] == 'E' {
        return (
            curr_score,
            if curr_score < best_score {
                curr_score
            } else {
                best_score
            },
        );
    }

    let possible_dirs = match dir {
        '<' => ['<', 'v', '^'],
        'v' => ['v', '>', '<'],
        '^' => ['^', '<', '>'],
        '>' => ['>', '^', 'v'],
        _ => panic!("Invalid direction"),
    };
    let penalty = [1, 1001, 1001];

    for (&d, p) in possible_dirs.iter().zip(penalty) {
        let dir_vec = match d {
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            '>' => (0, 1),
            _ => panic!("Invalid direction"),
        };

        let (n_i, n_j) = (
            (i as i32 + dir_vec.0) as usize,
            (j as i32 + dir_vec.1) as usize,
        );

        if maze[n_i][n_j] == '#' {
            continue;
        }

        let (_, n_best_score) = step(maze, ((n_i, n_j), d), curr_score + p, best_score, visited.clone());

        if n_best_score < best_score {
            best_score = n_best_score;
        }
    }

    (curr_score, best_score)
}

fn p1(input: &str) -> usize {
    let maze = parse(input);
    let start = find_start(&maze);

    step(&maze, (start, '>'), 0, usize::MAX, HashSet::new()).1
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

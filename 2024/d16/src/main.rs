use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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

fn render_maze(maze: &[Vec<char>], visited: &HashSet<(usize, usize)>) {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if visited.contains(&(i, j)) {
                print!("\x1b[31m{}\x1b[0m", cell);
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();
}

fn adder(a: (usize, usize), b: (i8, i8)) -> (usize, usize) {
    (
        (a.0 as i32 + b.0 as i32) as usize,
        (a.1 as i32 + b.1 as i32) as usize,
    )
}

fn solver(maze: &[Vec<char>], start_position: (usize, usize), start_direction: (i8, i8)) -> usize {
    let mut pq: BinaryHeap<Reverse<(usize, (usize, usize), (i8, i8))>> = BinaryHeap::new();
    pq.push(Reverse((0, start_position, start_direction)));

    let mut visited = HashSet::new();

    while let Some(Reverse((score, position, direction))) = pq.pop() {
        visited.insert((position, direction));

        let (i, j) = position;
        let (di, dj) = direction;

        if maze[i][j] == 'E' {
            return score;
        }

        for (c, (n_i, n_j), (n_di, n_dj)) in [
            (1, adder(position, direction), direction),
            (1000, position, (-dj, di)),
            (1000, position, (dj, -di)),
        ] {
            if maze[n_i][n_j] == '#' || visited.contains(&((n_i, n_j), (n_di, n_dj))) {
                continue;
            }

            pq.push(Reverse((score + c, (n_i, n_j), (n_di, n_dj))));
        }
    }

    usize::MAX
}

fn p1(input: &str) -> usize {
    let maze = parse(input);
    let start_position = find_start(&maze);

    solver(&maze, start_position, (0, 1))
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

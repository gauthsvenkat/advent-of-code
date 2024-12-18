use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::fs;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;
type Direction = (i8, i8);

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<&str> = line.trim().split(',').collect();

            (coords[0].parse().unwrap(), coords[1].parse().unwrap())
        })
        .collect()
}

fn create_grid(coords: &[Position]) -> Grid {
    let (x_max, y_max) = (
        coords.iter().map(|(x, _)| x).max().unwrap(),
        coords.iter().map(|(_, y)| y).max().unwrap(),
    );

    let mut grid = vec![vec!['.'; x_max + 1]; y_max + 1];

    for (x, y) in coords {
        grid[*y][*x] = '#';
    }

    grid
}

fn adder(position: Position, vector: Direction) -> (i32, i32) {
    (
        position.0 as i32 + vector.0 as i32,
        position.1 as i32 + vector.1 as i32,
    )
}

fn _render(grid: &Grid, position: Position) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (x, y) == position {
                print!("\x1b[31m{}\x1b[0m", cell);
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();
}

fn solver(grid: &Grid) -> usize {
    let mut pq: BinaryHeap<Reverse<(usize, Position)>> = BinaryHeap::new();
    pq.push(Reverse((0, (0, 0))));

    let (y_max, x_max) = (grid.len() - 1, grid[0].len() - 1);

    let mut seen: HashSet<Position> = HashSet::new();

    while let Some(Reverse((score, position))) = pq.pop() {
        let (x, y) = position;

        if x == x_max && y == y_max {
            return score;
        }

        for (nx, ny) in [
            adder(position, (-1, 0)), // left
            adder(position, (0, 1)),  // down
            adder(position, (0, -1)), // up
            adder(position, (1, 0)),  // right
        ] {
            if nx < 0 || ny < 0 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if nx > x_max || ny > y_max || grid[ny][nx] == '#' || seen.contains(&(nx, ny)) {
                continue;
            }

            seen.insert((nx, ny));
            pq.push(Reverse((score + 1, (nx, ny))));
        }
    }

    usize::MAX
}

fn p1(input: &str) -> usize {
    let coords = parse(input);
    let grid = create_grid(&coords[..1024]);

    solver(&grid)
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

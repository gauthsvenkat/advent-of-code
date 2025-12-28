use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    env, fs,
};

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

fn solver(grid: &Grid) -> Option<usize> {
    let mut pq: BinaryHeap<Reverse<(usize, Position)>> = BinaryHeap::new();
    pq.push(Reverse((0, (0, 0))));

    let (y_max, x_max) = (grid.len() - 1, grid[0].len() - 1);

    let mut seen: HashSet<Position> = HashSet::new();

    while let Some(Reverse((score, position))) = pq.pop() {
        let (x, y) = position;

        if x == x_max && y == y_max {
            return Some(score);
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

    None
}

fn p1(input: &str, limit: usize) -> usize {
    let coords = parse(input);
    let grid = create_grid(&coords[..limit]);

    solver(&grid).expect("No solution found")
}

fn binary_search(low: usize, high: usize, coords: &[Position]) -> usize {
    let mid = (low + high) / 2;

    let grid = create_grid(&coords[..mid]);

    if low == mid {
        mid
    } else if solver(&grid).is_some() {
        binary_search(mid, high, coords)
    } else {
        binary_search(low, mid, coords)
    }
}

fn p2(input: &str) -> String {
    let coords = parse(input);
    let first_unsolvable = binary_search(0, coords.len(), &coords);
    let coord = coords.get(first_unsolvable).unwrap();

    format!("{},{}", coord.0, coord.1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input, 1024)),
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
        assert_eq!(p1(EXAMPLE, 12), 22);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), "6,1");
    }
}

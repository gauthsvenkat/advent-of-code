use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    env, fs,
};

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

type Maze = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (i8, i8);
type History = Vec<Position>;
type State = (usize, Position, Direction, History);

fn _render(maze: &Maze, history: &History) {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if history.contains(&(i, j)) {
                print!("\x1b[31m{}\x1b[0m", cell);
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();
}

fn find_start(maze: &Maze) -> Position {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No start found");
}

fn adder(position: Position, vector: Direction) -> Position {
    (
        (position.0 as i32 + vector.0 as i32) as usize,
        (position.1 as i32 + vector.1 as i32) as usize,
    )
}

fn solver(maze: &Maze, start_position: Position, start_direction: Direction) -> (usize, usize) {
    let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    pq.push(Reverse((0, start_position, start_direction, Vec::new())));

    let mut visited = HashSet::new();
    let mut best_score: Option<usize> = None;
    let mut best_seats: HashSet<Position> = HashSet::new();

    while let Some(Reverse((score, position, direction, mut history))) = pq.pop() {
        visited.insert((position, direction));

        let (i, j) = position;
        let (di, dj) = direction;

        if maze[i][j] == 'E' {
            history.push(position);

            best_score = Some(score);
            history.iter().for_each(|&pos| {
                best_seats.insert(pos);
            });
        }

        if let Some(bs) = best_score {
            if score == bs {
                continue;
            } else {
                return (bs, best_seats.len());
            }
        }

        for (c, (n_i, n_j), (n_di, n_dj)) in [
            (1, adder(position, direction), direction),
            (1000, position, (-dj, di)),
            (1000, position, (dj, -di)),
        ] {
            if maze[n_i][n_j] == '#' || visited.contains(&((n_i, n_j), (n_di, n_dj))) {
                continue;
            }

            history.push(position);
            pq.push(Reverse((
                score + c,
                (n_i, n_j),
                (n_di, n_dj),
                history.clone(),
            )));
        }
    }

    (usize::MAX, usize::MIN)
}

fn p1(input: &str) -> usize {
    let maze = parse(input);
    let start_position = find_start(&maze);

    solver(&maze, start_position, (0, 1)).0
}

fn p2(input: &str) -> usize {
    let maze = parse(input);
    let start_position = find_start(&maze);

    solver(&maze, start_position, (0, 1)).1
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

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 7036);
        assert_eq!(p1(EXAMPLE2), 11048);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 45);
        assert_eq!(p2(EXAMPLE2), 64);
    }
}

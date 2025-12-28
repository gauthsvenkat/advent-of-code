use std::{collections::HashMap, env, fs};

type Maze = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (i8, i8);
type Path = Vec<Position>;

fn parse(input: &str) -> Maze {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn _render(maze: &Maze, history: &Path) {
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

fn adder(position: Position, vector: Direction) -> (i32, i32) {
    (
        position.0 as i32 + vector.0 as i32,
        position.1 as i32 + vector.1 as i32,
    )
}

fn dist(a: Position, b: Position) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn step(
    maze: &Maze,
    dimensions: Position,
    position: Position,
    direction: Direction,
    mut path: Path,
) -> Path {
    path.push(position);

    let (r_max, c_max) = dimensions;

    let (i, j) = position;
    let (di, dj) = direction;

    if maze[i][j] == 'E' {
        return path;
    }

    for (ndi, ndj) in [(di, dj), (dj, -di), (-dj, di)] {
        let (ni, nj) = adder(position, (ndi, ndj));

        if ni < 0 || nj < 0 {
            continue;
        }

        let (ni, nj) = (ni as usize, nj as usize);

        if ni > r_max || nj > c_max || maze[ni][nj] == '#' {
            continue;
        }

        return step(maze, dimensions, (ni, nj), (ndi, ndj), path);
    }

    panic!("No solution found");
}

fn cheats(solution: &Path, lasting: usize) -> HashMap<(Position, Position), usize> {
    let mut cheat_savings = HashMap::new();

    for s in 2..lasting + 1 {
        for (i, &position) in solution.iter().enumerate().take(solution.len() - s) {
            for (j, &other_position) in solution.iter().enumerate().skip(i + s) {
                if dist(position, other_position) == s && j - i > s {
                    cheat_savings
                        .entry((position, other_position))
                        .or_insert(j - i - s);
                }
            }
        }
    }

    cheat_savings
}

fn p1(input: &str, threshold: usize) -> usize {
    let maze = parse(input);
    let start = find_start(&maze);

    let solution = step(
        &maze,
        (maze.len() - 1, maze[0].len() - 1),
        start,
        (-1, 0),
        Vec::new(),
    );

    let cheat_savings = cheats(&solution, 2);

    cheat_savings
        .iter()
        .filter(|(_, &saved)| saved >= threshold)
        .count()
}

fn p2(input: &str, threshold: usize) -> usize {
    let maze = parse(input);
    let start = find_start(&maze);

    let solution = step(
        &maze,
        (maze.len() - 1, maze[0].len() - 1),
        start,
        (-1, 0),
        Vec::new(),
    );

    let cheat_savings = cheats(&solution, 20);

    cheat_savings
        .iter()
        .filter(|(_, &saved)| saved >= threshold)
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input, 100)),
        "p2" => println!("{}", p2(&input, 100)),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE, 1), 44);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE, 50), 285);
    }
}

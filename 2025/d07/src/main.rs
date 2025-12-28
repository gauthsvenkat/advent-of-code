use cached::proc_macro::cached;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type Position = (usize, usize);
type Grid = HashMap<Position, char>;
type Visited = HashSet<Position>;

fn _display_grid(grid: &Grid) {
    let max_row = grid.keys().map(|(i, _)| i).max().unwrap();
    let max_col = grid.keys().map(|(_, j)| j).max().unwrap();

    for i in 0..=*max_row {
        for j in 0..=*max_col {
            let c = grid.get(&(i, j)).unwrap();
            print!("{c}");
        }
        println!();
    }
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .collect()
}

fn count_splits(grid: &Grid, position: &Position, visited: &mut Visited) -> usize {
    if !grid.contains_key(position) || !visited.insert(*position) {
        return 0;
    }

    if *grid.get(position).unwrap() == '^' {
        let next_position_left = &(position.0, position.1 - 1);
        let next_position_right = &(position.0, position.1 + 1);

        1 + count_splits(grid, next_position_left, visited)
            + count_splits(grid, next_position_right, visited)
    } else {
        let next_position = &(position.0 + 1, position.1);

        count_splits(grid, next_position, visited)
    }
}

fn find_start(grid: &Grid) -> Position {
    for (&(i, j), &c) in grid {
        if c == 'S' {
            return (i, j);
        }
    }

    unreachable!("Should have found start by now");
}

fn p1(input: &str) -> usize {
    let grid = parse(input);

    count_splits(&grid, &find_start(&grid), &mut HashSet::new())
}

#[cached(key = "Position", convert = r#"{ *position }"#)]
fn count_timelines(grid: &Grid, position: &Position) -> usize {
    if !grid.contains_key(position) {
        return 1;
    }

    if *grid.get(position).unwrap() == '^' {
        let next_position_left = &(position.0, position.1 - 1);
        let next_position_right = &(position.0, position.1 + 1);

        count_timelines(grid, next_position_left) + count_timelines(grid, next_position_right)
    } else {
        let next_position = &(position.0 + 1, position.1);

        count_timelines(grid, next_position)
    }
}

fn p2(input: &str) -> usize {
    let grid = parse(input);

    count_timelines(&grid, &find_start(&grid))
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
        assert_eq!(p1(EXAMPLE), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 40);
    }
}

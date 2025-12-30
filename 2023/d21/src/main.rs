use std::{collections::HashMap, env, fs};

type Pos = (isize, isize);
type Grid = HashMap<Pos, char>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as isize, j as isize), ch))
        })
        .collect()
}

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

// Idea
// - fn reach(pos, 0, grid) -> 1
// - fn reach(pos, steps_remaining > 0, grid) -> sum(reach(valid_position, steps_remaining -1, grid) for valid position from pos)
// - cache on pos and steps_remaining for gainz

fn n_reach(pos: Pos, steps: usize, grid: &Grid) -> usize {
    if steps == 0 {
        return 1;
    }

    let (x, y) = pos;

    let next_positions: Vec<_> = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
        .into_iter()
        .filter(|p| {
            if let Some(ch) = grid.get(p) {
                *ch != '#'
            } else {
                false
            }
        })
        .collect();

    next_positions
        .into_iter()
        .map(|p| n_reach(p, steps - 1, grid))
        .sum()
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    _display_grid(&grid);
    let starting_position = grid
        .iter()
        .find_map(|(&p, &ch)| if ch == 'S' { Some(p) } else { None })
        .unwrap();

    n_reach(starting_position, 3, &grid)
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        // TODO:
        assert_eq!(p1(EXAMPLE), 0);
    }

    #[test]
    fn test_p2() {
        // TODO:
        assert_eq!(p2(EXAMPLE), 0);
    }
}

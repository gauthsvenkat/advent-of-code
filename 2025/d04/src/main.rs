use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Position = (isize, isize);

fn can_remove(grid: &[Vec<char>], pos: &Position) -> bool {
    let mut n: usize = 0;

    let offsets = vec![
        (-1, -1), // up-left,
        (-1, 0),  // up
        (-1, 1),  // up-right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // down-left
        (1, 0),   // down
        (1, 1),   // down-right
    ];

    for offset in offsets {
        let adj_pos_raw = (pos.0 + offset.0, pos.1 + offset.1);

        if (0..grid.len() as isize).contains(&adj_pos_raw.0)
            && (0..grid[0].len() as isize).contains(&adj_pos_raw.1)
            && grid[adj_pos_raw.0 as usize][adj_pos_raw.1 as usize] == '@'
        {
            n += 1;
        }
    }

    n < 4
}

fn p1(input: &str) -> usize {
    let parsed_input = parse(input);
    let mut acc = 0;
    for (i, row) in parsed_input.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '@' && can_remove(&parsed_input, &(i as isize, j as isize)) {
                acc += 1;
            }
        }
    }

    acc
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    let mut grid = parsed_input.to_owned();
    let mut acc = 0;

    loop {
        let mut loop_acc = 0;
        let mut to_remove: HashSet<Position> = HashSet::default();

        for (i, row) in grid.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch != '@' {
                    continue;
                }

                let pos = (i as isize, j as isize);

                if can_remove(&grid, &pos) {
                    loop_acc += 1;

                    to_remove.insert(pos);
                }
            }
        }

        for pos in to_remove {
            grid[pos.0 as usize][pos.1 as usize] = '.';
        }

        acc += loop_acc;

        if loop_acc == 0 {
            break;
        }
    }

    acc
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 13);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 43);
    }
}

use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

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

fn get_next_positions(current: HashSet<Pos>, grid: &Grid) -> HashSet<Pos> {
    current
        .into_iter()
        .flat_map(|curr| {
            let (x, y) = curr;

            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter(|pos| {
                    if let Some(ch) = grid.get(pos) {
                        *ch != '#'
                    } else {
                        false
                    }
                })
                .collect::<HashSet<Pos>>()
        })
        .collect()
}

// Idea
// - fn reach(pos, 0, grid) -> 1
// - fn reach(pos, steps_remaining > 0, grid) -> sum(reach(valid_position, steps_remaining -1, grid) for valid position from pos)
// - cache on pos and steps_remaining for gainz

// fn get_next(current: &Pos, previous: &Pos, grid: &Grid) -> HashSet<Pos> {
//     // Get all valid next positions.
//     // valid positions are anything that doesn't have a '#'
//     // or is not the previous position.
//     let (x, y) = *current;
//
//     vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
//         .into_iter()
//         .filter(|pos| {
//             if let Some(ch) = grid.get(pos) {
//                 *ch != '#' && pos != previous
//             } else {
//                 false
//             }
//         })
//         .collect()
// }

// fn n_reach(current: &Pos, previous: &Pos, steps: usize, grid: &Grid) -> usize {
//     if steps == 0 {
//         return 1;
//     }
//
//     get_next(current, previous, grid)
//         .iter()
//         .map(|pos| n_reach(pos, current, steps - 1, grid))
//         .sum()
//
//     // let (x, y) = pos;
//     //
//     // let next_positions: Vec<_> = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
//     //     .into_iter()
//     //     .filter(|p| {
//     //         if let Some(ch) = grid.get(p) {
//     //             *ch != '#'
//     //         } else {
//     //             false
//     //         }
//     //     })
//     //     .collect();
//     //
//     // next_positions
//     //     .into_iter()
//     //     .map(|p| n_reach(p, steps - 1, grid))
//     //     .sum()
// }

fn p1(input: &str) -> usize {
    let grid = parse(input);

    let starting_position = grid
        .iter()
        .find_map(|(&p, &ch)| if ch == 'S' { Some(p) } else { None })
        .unwrap();

    let mut positions = HashSet::from([starting_position]);

    for i in 0..64 {
        positions = get_next_positions(positions, &grid);
    }

    // dbg!(&positions);
    positions.len()

    // _display_grid(&grid);
    // let starting_position = grid
    //     .iter()
    //     .find_map(|(&p, &ch)| if ch == 'S' { Some(p) } else { None })
    //     .unwrap();
    //
    // n_reach(&starting_position, &starting_position, 6, &grid)
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
        assert_eq!(p1(EXAMPLE), 42);
    }

    #[test]
    fn test_p2() {
        // TODO:
        assert_eq!(p2(EXAMPLE), 0);
    }
}

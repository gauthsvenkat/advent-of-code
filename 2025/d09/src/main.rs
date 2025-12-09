use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;

type Position = (usize, usize);
type Positions = HashSet<Position>;

fn parse(input: &str) -> Positions {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn display_grid(points: &HashSet<(usize, usize)>) {
    if points.is_empty() {
        return;
    }

    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if points.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn area((x1, y1): &Position, (x2, y2): &Position) -> usize {
    (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1)
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_combinations()
        .map(|(r1, r2)| area(r1, r2))
        .max()
        .unwrap()
}

fn color_row(start: &Position, end: &Position) -> Positions {
    let (xs, ys) = start;
    let (xe, ye) = end;

    if ys != ye {
        dbg!(start, end);
        panic!("y coords are not same! start and end are not in the same row");
    }

    let (xs, xe) = if xs < xe { (xs, xe) } else { (xe, xs) };

    let mut positions = HashSet::new();

    for x in *xs..=*xe {
        positions.insert((x, *ys));
    }

    positions
}

fn color_col(start: &Position, end: &Position) -> Positions {
    let (xs, ys) = start;
    let (xe, ye) = end;

    if xs != xe {
        panic!("x coords are not same! start and end are not in the same column");
    }

    let (ys, ye) = if ys < ye { (ys, ye) } else { (ye, ys) };

    let mut positions = HashSet::new();

    for y in *ys..=*ye {
        positions.insert((*xs, y));
    }

    positions
}

// fn color(positions: &Positions) -> Positions {
//     let colored_rows: Positions = positions
//         .iter()
//         .tuple_combinations()
//         .filter(|((_, y1), (_, y2))| y1 == y2)
//         .flat_map(|(start, end)| color_row(start, end))
//         .collect();
//
//     let colored_columns: Positions = positions
//         .iter()
//         .tuple_combinations()
//         .filter(|((x1, _), (x2, _))| x1 == x2)
//         .flat_map(|(start, end)| color_col(start, end))
//         .collect();
//
//     let mut colored = HashSet::new();
//
//     colored.extend(colored_rows);
//     colored.extend(colored_columns);
//
//     colored
// }

// Idea:
// - find outer edges
// - fn to check if a point is inside the area (point in polygon algo)

fn p2(input: &str) -> usize {
    let positions = parse(input);
    // display_grid(&positions);

    let colored = color(&positions);
    display_grid(&colored);

    // let colored = color(colored);
    // display_grid(&colored);

    todo!()

    // positions
    //     .iter()
    //     .tuple_combinations()
    //     .filter(|((x1, y1), (x2, y2))| {
    //         let p3 = (*x1, *y2);
    //         let p4 = (*x2, *y1);
    //
    //         positions.contains(&p3) && positions.contains(&p4)
    //     })
    //     .map(|(r1, r2)| area(r1, r2))
    //     .max()
    //     .unwrap()
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
        assert_eq!(p1(EXAMPLE), 50);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 24);
    }
}

use itertools::Itertools;
use std::{env, fs, ops::Div};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left(usize),
    Down(usize),
    Up(usize),
    Right(usize),
}

use Direction as Dir;

impl Direction {
    fn from(dir: char, dist: usize) -> Dir {
        match dir {
            'L' => Dir::Left(dist),
            'D' => Dir::Down(dist),
            'U' => Dir::Up(dist),
            'R' => Dir::Right(dist),
            dir => panic!("Received invalid direction {dir}"),
        }
    }
}

#[derive(Debug)]
struct Trench {
    direction: Direction,
    color: String,
}

fn parse(input: &str) -> Vec<Trench> {
    input
        .lines()
        .map(|line| {
            if let Some((dir, dist, color)) = line.split_whitespace().collect_tuple() {
                let (dir, dist, color) = (
                    dir.chars().next().unwrap(),
                    dist.parse().unwrap(),
                    color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap(),
                );
                Trench {
                    direction: Direction::from(dir, dist),
                    color: color.to_string(),
                }
            } else {
                panic!("Couldn't parse line")
            }
        })
        .collect()
}

type Point = (usize, usize);

fn get_points(directions: &[Direction]) -> Vec<Point> {
    let mut curr = (0, 0);
    let mut points = Vec::new();

    for direction in directions {
        let (x, y) = curr;

        curr = match direction {
            Dir::Left(d) => (x - d, y),
            Dir::Down(d) => (x, y + d),
            Dir::Up(d) => (x, y - d),
            Dir::Right(d) => (x + d, y),
        };

        points.push(curr);
    }

    points
}

fn area(points: &[Point]) -> usize {
    // Shoelace algorithm
    let interior_points = points
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| {
            let (x1, y1) = (a.0 as isize, a.1 as isize);
            let (x2, y2) = (b.0 as isize, b.1 as isize);

            (x1 * y2) - (x2 * y1)
        })
        .sum::<isize>()
        .div(2)
        .unsigned_abs();

    let boundary_points = points
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| {
            let (x1, y1) = (a.0 as isize, a.1 as isize);
            let (x2, y2) = (b.0 as isize, b.1 as isize);

            //in priciple, one of these terms should be 0, since the
            //edges are either horizontal or vertical.
            x1.abs_diff(x2) + y1.abs_diff(y2)
        })
        .sum::<usize>();

    // pick's theorem
    interior_points + boundary_points.div(2) + 1
}

fn p1(input: &str) -> usize {
    let trenches = parse(input);
    let directions: Vec<Direction> = trenches.iter().map(|trench| trench.direction).collect();
    let points = get_points(&directions);

    area(&points)
}

fn p2(input: &str) -> usize {
    let trenches = parse(input);

    let directions: Vec<Direction> = trenches
        .iter()
        .map(|trench| {
            let dist = usize::from_str_radix(&trench.color[..5], 16).unwrap();

            let dir = match trench.color.chars().nth(5).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                dir => panic!("Received invalid direction {dir}"),
            };

            Dir::from(dir, dist)
        })
        .collect();

    let points = get_points(&directions);

    area(&points)
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
        assert_eq!(p1(EXAMPLE), 62);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 952408144115);
    }
}

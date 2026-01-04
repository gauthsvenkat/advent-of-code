use std::{collections::HashMap, env, fs};

use Line::*;

type Point = (usize, usize);

#[derive(Debug)]
enum Line {
    Horizontal { start: Point, end: Point },
    Vertical { start: Point, end: Point },
    Other { start: Point, end: Point },
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();

            let start = start.split_once(',').unwrap();
            let end = end.split_once(',').unwrap();

            let (xs, ys): (usize, usize) = (start.0.parse().unwrap(), start.1.parse().unwrap());
            let (xe, ye): (usize, usize) = (end.0.parse().unwrap(), end.1.parse().unwrap());

            if xs == xe {
                Vertical {
                    start: (xs, ys),
                    end: (xe, ye),
                }
            } else if ys == ye {
                Horizontal {
                    start: (xs, ys),
                    end: (xe, ye),
                }
            } else {
                Other {
                    start: (xs, ys),
                    end: (xe, ye),
                }
            }
        })
        .collect()
}

fn populate(lines: &[Line]) -> HashMap<Point, usize> {
    let mut grid = HashMap::new();

    for line in lines {
        match line {
            Vertical { start, end } => {
                let (xs, ys) = start;
                let (_, ye) = end;

                let (s, e) = if ys < ye { (ys, ye) } else { (ye, ys) };

                for y in *s..=*e {
                    *grid.entry((*xs, y)).or_default() += 1;
                }
            }
            Horizontal { start, end } => {
                let (xs, ys) = start;
                let (xe, _) = end;

                let (s, e) = if xs < xe { (xs, xe) } else { (xe, xs) };

                for x in *s..=*e {
                    *grid.entry((x, *ys)).or_default() += 1;
                }
            }
            Other { start, end } => {
                let (xs, ys) = start;
                let (xe, ye) = end;

                let xx: Vec<_> = if *xs < *xe {
                    (*xs..=*xe).collect()
                } else {
                    (*xe..=*xs).rev().collect()
                };

                let yy: Vec<_> = if *ys < *ye {
                    (*ys..=*ye).collect()
                } else {
                    (*ye..=*ys).rev().collect()
                };

                for (x, y) in xx.iter().zip(yy) {
                    *grid.entry((*x, y)).or_default() += 1;
                }
            }
        }
    }

    grid
}

fn p1(input: &str) -> usize {
    let lines: Vec<Line> = parse(input)
        .into_iter()
        .filter(|line| matches!(line, Horizontal { .. } | Vertical { .. }))
        .collect();

    let grid = populate(&lines);

    grid.values().filter(|&&v| v > 1).count()
}

fn p2(input: &str) -> usize {
    let lines = parse(input);
    let grid = populate(&lines);

    grid.values().filter(|&&v| v > 1).count()
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
        assert_eq!(p1(EXAMPLE), 5);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 12);
    }
}

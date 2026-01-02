use std::{env, fs};

type Point = (usize, usize);

use Action::*;

#[derive(Debug)]
enum Action {
    Toggle,
    On,
    Off,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    from: Point,
    to: Point,
}

fn parse(input: &str) -> Vec<Instruction> {
    let input = input.replace("turn ", "").replace("through ", "");

    input
        .lines()
        .map(|line| {
            let (action, rest) = line.split_once(' ').unwrap();
            let (from, to) = rest.split_once(' ').unwrap();

            let from = from.split_once(',').unwrap();
            let to = to.split_once(',').unwrap();

            let from: Point = (from.0.parse().unwrap(), from.1.parse().unwrap());
            let to: Point = (to.0.parse().unwrap(), to.1.parse().unwrap());

            let action = match action {
                "toggle" => Toggle,
                "on" => On,
                "off" => Off,
                s => panic!("Invalid action {s}"),
            };

            Instruction { action, from, to }
        })
        .collect()
}

fn p1(input: &str) -> usize {
    let instructions = parse(input);
    let mut grid = vec![vec![false; 1000]; 1000];

    for instruction in &instructions {
        let Instruction { action, from, to } = &instruction;
        let (xs, ys) = from;
        let (xe, ye) = to;

        #[allow(clippy::needless_range_loop)]
        for i in *xs..=*xe {
            for j in *ys..=*ye {
                grid[i][j] = match action {
                    Toggle => !grid[i][j],
                    On => true,
                    Off => false,
                }
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&v| v).count())
        .sum()
}

fn p2(input: &str) -> usize {
    let instructions = parse(input);
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for instruction in &instructions {
        let Instruction { action, from, to } = &instruction;
        let (xs, ys) = from;
        let (xe, ye) = to;

        #[allow(clippy::needless_range_loop)]
        for i in *xs..=*xe {
            for j in *ys..=*ye {
                grid[i][j] = match action {
                    Toggle => grid[i][j].saturating_add(2),
                    On => grid[i][j].saturating_add(1),
                    Off => grid[i][j].saturating_sub(1),
                }
            }
        }
    }

    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
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
        assert_eq!(p1(EXAMPLE), 998996);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 1001996);
    }
}

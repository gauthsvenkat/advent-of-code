use std::{collections::HashSet, env, fs, ops::Div};

fn parse(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_once(' ').unwrap();

            let dir = dir.chars().next().unwrap();
            let steps: usize = steps.parse().unwrap();

            (dir, steps)
        })
        .collect()
}

type Point = (i32, i32);

fn simulate(rope_length: usize, motions: &[(char, usize)]) -> usize {
    let mut rope: Vec<Point> = vec![(0, 0); rope_length];
    let mut visited: HashSet<Point> = HashSet::from([(0, 0)]);

    for (dir, step) in motions {
        let vector = match dir {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            dir => panic!("Invalid direction {dir}"),
        };

        for _ in 0..*step {
            rope[0] = ((rope[0].0 + vector.0), (rope[0].1 + vector.1));

            for i in 0..rope.len() - 1 {
                let (a, mut b) = (rope[i], rope[i + 1]);

                let (dx, dy) = ((a.0 - b.0), (a.1 - b.1));

                if dx.abs() > 1 || dy.abs() > 1 {
                    if dx == 0 {
                        b.1 += dy.div(2);
                    } else if dy == 0 {
                        b.0 += dx.div(2);
                    } else {
                        b.0 += if dx > 0 { 1 } else { -1 };
                        b.1 += if dy > 0 { 1 } else { -1 };
                    }

                    rope[i + 1] = b;
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn p1(input: &str) -> usize {
    let motions = parse(input);

    simulate(2, &motions)
}

fn p2(input: &str) -> usize {
    let motions = parse(input);

    simulate(10, &motions)
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
    fn test_p1_example1() {
        assert_eq!(p1(EXAMPLE1), 13);
    }

    #[test]
    fn test_p1_example2() {
        assert_eq!(p1(EXAMPLE2), 88);
    }

    #[test]
    fn test_p2_example1() {
        assert_eq!(p2(EXAMPLE1), 1);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(p2(EXAMPLE2), 36);
    }
}

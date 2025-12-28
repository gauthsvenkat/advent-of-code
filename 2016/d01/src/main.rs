use std::{collections::HashSet, env, fs};

type Instruction = (char, isize);
type Point = (isize, isize);

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|seq| {
            let seq = seq.trim();

            let dir = seq.chars().next().unwrap();
            let dist: isize = seq[1..].parse().unwrap();

            (dir, dist)
        })
        .collect()
}

fn follow_instruction(
    loc: &Point,
    facing: &Point,
    instruction: &Instruction,
    mut visited: HashSet<Point>,
) -> (Point, Point, HashSet<Point>, Option<Point>) {
    let (x, y) = loc;
    let (dx, dy) = facing;

    let mut revisit = None;

    let (dir, dist) = instruction;

    let (dx, dy) = match dir {
        'R' => (*dy, -dx),
        'L' => (-dy, *dx),
        c => panic!("Invalid direction {c}"),
    };

    for step in 1..=*dist {
        let p = (x + dx * step, y + dy * step);
        if visited.contains(&p) {
            revisit = revisit.or(Some(p));
        } else {
            visited.insert(p);
        }
    }

    let (nx, ny) = (x + dx * dist, y + dy * dist);

    ((nx, ny), (dx, dy), visited, revisit)
}

fn p1(input: &str) -> isize {
    let instructions = parse(input);

    let mut loc = (0, 0);
    let mut facing = (0, 1);

    for instruction in instructions {
        (loc, facing, _, _) = follow_instruction(&loc, &facing, &instruction, HashSet::new());
    }

    loc.0.abs() + loc.1.abs()
}

fn p2(input: &str) -> isize {
    let instructions = parse(input);

    let mut loc = (0, 0);
    let mut facing = (0, 1);

    let mut visited: HashSet<Point> = HashSet::new();
    let mut revisit;

    for instruction in instructions {
        (loc, facing, visited, revisit) = follow_instruction(&loc, &facing, &instruction, visited);

        if let Some(p) = revisit {
            return p.0.abs() + p.1.abs();
        }
    }

    unreachable!("Should have revisted some point by now!")
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
    const EXAMPLE3: &str = include_str!("../eg3.txt");
    const EXAMPLE4: &str = include_str!("../eg4.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 5);
        assert_eq!(p1(EXAMPLE2), 2);
        assert_eq!(p1(EXAMPLE3), 12);
        assert_eq!(p1(EXAMPLE4), 8);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE4), 4);
    }
}

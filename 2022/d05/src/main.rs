use std::{
    collections::{BTreeMap, VecDeque},
    env, fs,
};

#[allow(clippy::ptr_arg)]
fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n_col = matrix[0].len();

    let mut t_matrix = vec![vec![]; n_col];

    for row in matrix.iter() {
        for j in 0..row.len() {
            t_matrix[j].push(row[j]);
        }
    }

    t_matrix
}

type Stacks = BTreeMap<usize, VecDeque<char>>;

fn parse_stacks_block(input: &str) -> Stacks {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let chars = transpose(&chars);

    let mut stacks = BTreeMap::new();

    for row in chars.iter() {
        if !row.iter().any(|c| c.is_alphanumeric()) {
            continue;
        }

        let mut stack: VecDeque<char> = row
            .iter()
            .filter(|c| c.is_alphanumeric())
            .copied()
            .collect();
        let id = stack.pop_back().unwrap().to_digit(10).unwrap() as usize;

        stacks.insert(id, stack);
    }

    stacks
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_moves_block(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .filter_map(|s| {
                    if s.chars().all(|c| c.is_numeric()) {
                        Some(s.parse::<usize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect();

            Move {
                count: nums[0],
                from: nums[1],
                to: nums[2],
            }
        })
        .collect()
}

fn parse(input: &str) -> (Stacks, Vec<Move>) {
    let (crate_block, moves_block) = input.split_once("\n\n").unwrap();

    let stacks = parse_stacks_block(crate_block);
    let moves = parse_moves_block(moves_block);

    (stacks, moves)
}

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

fn do_move(mut stacks: Stacks, mov: &Move, crane: &Crane) -> Stacks {
    let (id_from, mut stack_from) = stacks.remove_entry(&mov.from).unwrap();

    let (id_to, mut stack_to) = stacks.remove_entry(&mov.to).unwrap();

    let buffer = stack_from.drain(..mov.count);
    let buffer: Vec<char> = match crane {
        Crane::CrateMover9000 => buffer.collect(),
        Crane::CrateMover9001 => buffer.rev().collect(),
    };

    for c in buffer {
        stack_to.push_front(c);
    }

    stacks.insert(id_to, stack_to);
    stacks.insert(id_from, stack_from);

    stacks
}

fn do_moves(mut stacks: Stacks, moves: &[Move], crane: &Crane) -> Stacks {
    for mov in moves {
        stacks = do_move(stacks, mov, crane);
    }

    stacks
}

fn tops(stacks: Stacks) -> String {
    stacks
        .into_values()
        .map(|stack| *stack.front().unwrap())
        .collect()
}

fn p1(input: &str) -> String {
    let (stacks, moves) = parse(input);

    let stacks = do_moves(stacks, &moves, &Crane::CrateMover9000);

    tops(stacks)
}

fn p2(input: &str) -> String {
    let (stacks, moves) = parse(input);

    let stacks = do_moves(stacks, &moves, &Crane::CrateMover9001);

    tops(stacks)
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
        assert_eq!(p1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), "MCD");
    }
}

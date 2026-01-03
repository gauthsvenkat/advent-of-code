use std::{collections::HashMap, env, fs};

type Point = (usize, usize);

#[derive(Debug)]
struct Board {
    left: HashMap<usize, Point>,
    drawn: HashMap<usize, Point>,
    won: bool,
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut blocks = input.split("\n\n");
    let numbers: Vec<usize> = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<HashMap<usize, Point>> = blocks
        .map(|block| {
            block
                .lines()
                .enumerate()
                .flat_map(move |(i, row)| {
                    row.split_whitespace()
                        .enumerate()
                        .map(move |(j, n)| (n.parse().unwrap(), (i, j)))
                })
                .collect()
        })
        .collect();

    let boards = boards
        .into_iter()
        .map(|b| Board {
            left: b,
            drawn: HashMap::new(),
            won: false,
        })
        .collect();

    (numbers, boards)
}

impl Board {
    fn draw(&mut self, num: usize) -> bool {
        if self.won {
            // Can you continue winning if you've already won?
            return false;
        }

        if let Some(pos) = self.left.remove(&num) {
            self.drawn.insert(num, pos);

            for i in 0..5 {
                // Full row
                if self.drawn.values().filter(|(x, _)| *x == i).count() == 5 {
                    self.won = true;
                    return true;
                }
                // Full column
                if self.drawn.values().filter(|(_, y)| *y == i).count() == 5 {
                    self.won = true;
                    return true;
                }
            }
        }

        false
    }
}

fn play(boards: &mut [Board], number: usize) -> Vec<usize> {
    let mut won_idxs = Vec::new();

    for (i, board) in boards.iter_mut().enumerate() {
        if board.draw(number) {
            won_idxs.push(i);
        }
    }

    won_idxs
}

fn p1(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for number in numbers {
        let mut maybe_won = play(&mut boards, number);
        if let Some(board_idx) = maybe_won.pop() {
            return boards[board_idx].left.keys().sum::<usize>() * number;
        }
    }

    unreachable!("Some board should have won by now!")
}

fn p2(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    let mut won = Vec::new();

    for number in numbers {
        let maybe_won = play(&mut boards, number);

        won.extend(maybe_won);

        if won.len() == boards.len() {
            return boards[*won.last().unwrap()].left.keys().sum::<usize>() * number;
        }
    }

    unreachable!("Some board should have won by now!")
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
        assert_eq!(p1(EXAMPLE), 4512);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 1924);
    }
}

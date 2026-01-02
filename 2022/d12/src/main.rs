use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    env, fs,
    ops::Sub,
};

type Point = (isize, isize);
type Grid = HashMap<Point, char>;

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

fn elevation_gain(curr: char, dest: char) -> isize {
    let curr = if curr == 'S' { 'a' } else { curr };
    let dest = if dest == 'E' { 'z' } else { dest };

    let curr = (curr as u8) as isize;
    let dest = (dest as u8) as isize;

    dest.sub(curr)
}

fn traverse(pos: &Point, grid: &Grid) -> usize {
    let mut pq = BinaryHeap::from([Reverse((0, *pos))]);
    let mut seen = HashSet::new();

    while let Some(Reverse((cost, pos))) = pq.pop() {
        if *grid.get(&pos).unwrap() == 'E' {
            return cost;
        }

        if seen.contains(&pos) {
            continue;
        } else {
            seen.insert(pos);
        }

        let (x, y) = pos;

        for next in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
            if !grid.contains_key(&next) {
                continue;
            }

            if elevation_gain(*grid.get(&pos).unwrap(), *grid.get(&next).unwrap()) > 1 {
                continue;
            }

            pq.push(Reverse((cost + 1, next)));
        }
    }

    usize::MAX
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    let (pos, _) = grid.iter().find(|(_, &ch)| ch == 'S').unwrap();
    traverse(pos, &grid)
}

fn p2(input: &str) -> usize {
    let grid = parse(input);

    grid.iter()
        .filter(|(_, &ch)| ch == 'a' || ch == 'S')
        .map(|(pos, _)| traverse(pos, &grid))
        .min()
        .unwrap()
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
        assert_eq!(p1(EXAMPLE), 31);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 29);
    }
}

use std::{
    collections::{BinaryHeap, HashSet},
    env, fs,
};

struct Map {
    grid: Vec<Vec<isize>>,
    n_rows: isize,
    n_cols: isize,
}

impl Map {
    fn get(&self, i: isize, j: isize) -> Option<isize> {
        if i < 0 || j < 0 || i >= self.n_rows || j >= self.n_cols {
            return None;
        }

        Some(self.grid[i as usize][j as usize])
    }
}

fn parse(input: &str) -> Map {
    let grid: Vec<Vec<isize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    Map {
        n_rows: grid.len() as isize,
        n_cols: grid[0].len() as isize,
        grid,
    }
}

// (0, -1) => Direction::Left,
// (1, 0) => Direction::Down,
// (-1, 0) => Direction::Up,
// (0, 1) => Direction::Right,
fn travel(
    map: &Map,
    (si, sj): (isize, isize),
    (min_consecutive_blocks, max_consecutive_blocks): (isize, isize),
) -> isize {
    // (c, i, j, di, dj, dc)
    let mut pq = BinaryHeap::from([(0, si, sj, 0, 0, 0)]);
    let mut seen = HashSet::new();

    while let Some((cost, i, j, di, dj, n)) = pq.pop() {
        if seen.contains(&(i, j, di, dj, n)) {
            continue;
        }

        seen.insert((i, j, di, dj, n));

        if (i, j) == (map.n_rows - 1, map.n_cols - 1) && n >= min_consecutive_blocks {
            return -cost;
        }

        if n < max_consecutive_blocks && (di, dj) != (0, 0) {
            if let Some(cell_cost) = map.get(i + di, j + dj) {
                pq.push((cost - cell_cost, i + di, j + dj, di, dj, n + 1));
            }
        }

        for (ndi, ndj) in [(0, -1), (1, 0), (-1, 0), (0, 1)] {
            if (ndi, ndj) == (di, dj)
                || (ndi, ndj) == (-di, -dj)
                || ((di, dj) != (0, 0) && n < min_consecutive_blocks)
            {
                continue;
            }

            if let Some(cell_cost) = map.get(i + ndi, j + ndj) {
                pq.push((cost - cell_cost, i + ndi, j + ndj, ndi, ndj, 1));
            }
        }
    }

    isize::MAX
}

fn p1(input: &str) -> usize {
    let map = parse(input);
    travel(&map, (0, 0), (0, 3)) as usize
}

fn p2(input: &str) -> usize {
    let map = parse(input);
    travel(&map, (0, 0), (4, 10)) as usize
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE1), 102);
    }

    #[test]
    fn test_p2_example1() {
        assert_eq!(p2(EXAMPLE1), 94);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(p2(EXAMPLE2), 71);
    }
}

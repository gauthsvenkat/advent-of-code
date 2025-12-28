use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn group_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut record: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                record.entry(cell).or_default().push((i as i32, j as i32));
            }
        }
    }
    record
}

fn in_grid(pos: (i32, i32), dims: (i32, i32)) -> bool {
    let (x, y) = pos;
    let (n, m) = dims;

    0 <= x && 0 <= y && x < n && y < m
}

fn pos_sub(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    (p1.0 - p2.0, p1.1 - p2.1)
}

fn antinodes_part1(p1: (i32, i32), p2: (i32, i32), dims: (i32, i32)) -> Vec<(i32, i32)> {
    let (dx, dy) = pos_sub(p1, p2);

    vec![(p1.0 + dx, p1.1 + dy), (p2.0 - dx, p2.1 - dy)]
        .into_iter()
        .filter(|&antinode| in_grid(antinode, dims))
        .collect()
}

fn p1(input: &str) -> usize {
    let grid = parse(input);
    let record = group_antennas(&grid);
    let dims = (grid.len() as i32, grid[0].len() as i32);

    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in record.values() {
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                antinodes_part1(pos1, pos2, dims)
                    .iter()
                    .for_each(|&antinode| {
                        unique_antinodes.insert(antinode);
                    });
            }
        }
    }

    (unique_antinodes).len()
}

fn antinodes_part2(p1: (i32, i32), p2: (i32, i32), dims: (i32, i32)) -> Vec<(i32, i32)> {
    let (dx, dy) = pos_sub(p1, p2);

    let harmonics_p1 = (0..)
        .map(|n| (p1.0 + n * dx, p1.1 + n * dy))
        .take_while(|&pos| in_grid(pos, dims));

    let harmonics_p2 = (0..)
        .map(|n| (p2.0 - n * dx, p2.1 - n * dy))
        .take_while(|&pos| in_grid(pos, dims));

    harmonics_p1.chain(harmonics_p2).collect()
}

fn p2(input: &str) -> usize {
    let grid = parse(input);
    let record = group_antennas(&grid);
    let dims = (grid.len() as i32, grid[0].len() as i32);

    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in record.values() {
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                antinodes_part2(pos1, pos2, dims)
                    .iter()
                    .for_each(|&antinode| {
                        unique_antinodes.insert(antinode);
                    });
            }
        }
    }

    (unique_antinodes).len()
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
        assert_eq!(p1(EXAMPLE), 14);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 34);
    }
}

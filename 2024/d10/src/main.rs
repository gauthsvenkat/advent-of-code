use std::{collections::HashSet, env, fs};

type Position = (usize, usize);
type Path = Vec<Position>;
type Summits = HashSet<Position>;
type Paths = HashSet<Path>;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_trailheads(map: &[Vec<u32>]) -> Vec<Position> {
    let mut trailheads = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                trailheads.push((i, j));
            }
        }
    }

    trailheads
}

fn hike(
    map: &Vec<Vec<u32>>,
    map_dims: Position,
    position: Position,
    mut unique_summits: Summits,
    mut path: Path,
    mut unique_paths: Paths,
) -> (Summits, Paths) {
    let (x, y) = position;
    let height = map[x][y];

    path.push(position);

    if height == 9 {
        unique_summits.insert(position);
        unique_paths.insert(path);
        return (unique_summits, unique_paths);
    }

    let (max_x, max_y) = map_dims;

    let mut to_visit_next: Vec<(usize, usize)> = Vec::new();

    if y > 0 && (map[x][y - 1] == height + 1) {
        to_visit_next.push((x, y - 1));
    }

    if x < max_x && (map[x + 1][y] == height + 1) {
        to_visit_next.push((x + 1, y));
    }

    if x > 0 && (map[x - 1][y] == height + 1) {
        to_visit_next.push((x - 1, y));
    }

    if y < max_y && (map[x][y + 1] == height + 1) {
        to_visit_next.push((x, y + 1));
    }

    for &next_position in to_visit_next.iter() {
        (unique_summits, unique_paths) = hike(
            map,
            map_dims,
            next_position,
            unique_summits,
            path.clone(),
            unique_paths,
        );
    }

    (unique_summits, unique_paths)
}

fn p1(input: &str) -> usize {
    let map = parse(input);
    let map_dims = (map.len() - 1, map[0].len() - 1);
    let trailheads = get_trailheads(&map);

    trailheads
        .iter()
        .map(|&trailhead| {
            hike(
                &map,
                map_dims,
                trailhead,
                HashSet::new(),
                Vec::new(),
                HashSet::new(),
            )
            .0
            .len()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let map = parse(input);
    let map_dims = (map.len() - 1, map[0].len() - 1);
    let trailheads = get_trailheads(&map);

    trailheads
        .iter()
        .map(|&trailhead| {
            hike(
                &map,
                map_dims,
                trailhead,
                HashSet::new(),
                Vec::new(),
                HashSet::new(),
            )
            .1
            .len()
        })
        .sum()
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
        assert_eq!(p1(EXAMPLE), 36);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 81);
    }
}

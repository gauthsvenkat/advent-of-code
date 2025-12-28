use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env, fs,
};

type Point = (usize, usize);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn area((x1, y1): &Point, (x2, y2): &Point) -> usize {
    (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1)
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_combinations()
        .map(|(r1, r2)| area(r1, r2))
        .max()
        .unwrap()
}

fn rasterize(red_tiles: &[Point]) -> HashSet<Point> {
    red_tiles
        .iter()
        .circular_tuple_windows()
        .flat_map(|(p1, p2)| get_edge(p1, p2))
        .collect()
}

fn get_edge(point_a: &Point, point_b: &Point) -> HashSet<Point> {
    // Get all the `Point`s along an edge defined by `point_a` and `point_b`
    let (x_a, y_a) = point_a;
    let (x_b, y_b) = point_b;

    if x_a == x_b {
        let start = y_a.min(y_b);
        let end = y_a.max(y_b);

        (*start..=*end).map(|y| (*x_a, y)).collect()
    } else if y_a == y_b {
        let start = x_a.min(x_b);
        let end = x_a.max(x_b);

        (*start..=*end).map(|x| (x, *y_a)).collect()
    } else {
        panic!(
            "points {:?} {:?} are not along the same edge!",
            point_a, point_b
        );
    }
}

fn get_edges(corner_a: &Point, corner_b: &Point) -> HashSet<Point> {
    // Get all the `Point`s of a rectangle, given by two adjacent corners
    // `corner_a` and `corner_b`.

    let (x_a, y_a) = corner_a;
    let (x_b, y_b) = corner_b;

    let corner_c = (*x_a, *y_b);
    let corner_d = (*x_b, *y_a);

    get_edge(corner_a, &corner_c)
        .into_iter()
        .chain(get_edge(corner_a, &corner_d))
        .chain(get_edge(corner_b, &corner_c))
        .chain(get_edge(corner_b, &corner_d))
        .collect()
}
fn flood_fill(point: &Point, rasterized_shape: &HashSet<Point>) -> HashSet<Point> {
    let mut to_visit = vec![*point];
    let mut filled = HashSet::new();

    while let Some((x, y)) = to_visit.pop() {
        if filled.contains(&(x, y)) || rasterized_shape.contains(&(x, y)) {
            continue;
        }

        filled.insert((x, y));

        to_visit.extend([
            (x.saturating_sub(1), y),
            (x, y + 1),
            (x, y.saturating_sub(1)),
            (x + 1, y),
        ]);
    }

    filled
}
fn find_best_rectangle(
    compressed_shape: &HashSet<Point>,
    compressed_red_tiles: &[Point],
    compressed_to_original_map: &HashMap<&Point, Point>,
) -> usize {
    let mut best_known_area: Option<usize> = None;

    for (corner_a, corner_b) in compressed_red_tiles.iter().tuple_combinations() {
        let area = area(
            compressed_to_original_map.get(corner_a).unwrap(),
            compressed_to_original_map.get(corner_b).unwrap(),
        );

        if let Some(best_area) = best_known_area {
            if area > best_area
                && get_edges(corner_a, corner_b)
                    .iter()
                    .all(|p| compressed_shape.contains(p))
            {
                best_known_area = Some(area);
            }
        } else {
            best_known_area = Some(area);
        }
    }

    best_known_area.unwrap_or(0)
}

fn compress_coordinates(points: &[Point]) -> Vec<Point> {
    let xs: BTreeSet<_> = points.iter().map(|(x, _)| x).collect();
    let ys: BTreeSet<_> = points.iter().map(|(_, y)| y).collect();

    // even spaces are occupied by the points in the original coordinates
    // while the odd spaces represent the compressed empty spaces.
    let new_xs: BTreeMap<_, _> = xs.iter().enumerate().map(|(i, x)| (x, i * 2)).collect();
    let new_ys: BTreeMap<_, _> = ys.iter().enumerate().map(|(i, y)| (y, i * 2)).collect();

    points
        .iter()
        .map(|(x, y)| (*new_xs.get(&x).unwrap(), *new_ys.get(&y).unwrap()))
        .collect()
}

fn p2(input: &str, seed: Point) -> usize {
    let red_tiles = parse(input);

    let compressed_red_tiles = compress_coordinates(&red_tiles);

    let compressed_to_original_map: HashMap<_, _> =
        compressed_red_tiles.iter().zip(red_tiles).collect();

    let compressed_rasterized_shape = rasterize(&compressed_red_tiles);

    let compressed_shape: HashSet<_> = flood_fill(&seed, &compressed_rasterized_shape)
        .into_iter()
        .chain(compressed_rasterized_shape)
        .collect();

    find_best_rectangle(
        &compressed_shape,
        &compressed_red_tiles,
        &compressed_to_original_map,
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input, (250, 300))),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 50);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE, (5, 3)), 24);
    }
}

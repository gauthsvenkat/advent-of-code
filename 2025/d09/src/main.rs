use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;

type Point = (usize, usize);
type Points = HashSet<Point>;
type Edge<'a> = (&'a Point, &'a Point);
type Edges<'a> = HashSet<Edge<'a>>;

fn parse(input: &str) -> Points {
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

// Idea:
// - find outer edges
// - fn to check if a point is inside the area (point in polygon algo) TODO:
//      - Idea:
//      - a point is inside if its projection in all 4 directions intersect with some edge.
//          Idea: How to find intersection
//              - Take the point, project it in a direction and see if there is an edge that
//              intersects
// - take all pairs of points (similar to p1), check if the other two points are inside the area
// (filter), then find the rect

fn get_vertical_edge<'a>(p1: &'a Point, p2: &'a Point) -> Option<Edge<'a>> {
    //no edge if same point
    if p1 == p2 {
        return None;
    }

    // vertical edge if on same column
    if p1.0 == p2.0 {
        return Some((p1, p2));
    }

    None
}

fn get_vertical_edges(points: &Points) -> Edges<'_> {
    points
        .iter()
        .tuple_combinations()
        .filter_map(|(p1, p2)| get_vertical_edge(p1, p2))
        .collect()
}

fn is_point_on_edge(p: &Point, edge: &Edge) -> bool {
    let (p1, p2) = edge;

    // Check if point is on vertical edge (same column)
    if p.0 == p1.0 {
        let (min_y, max_y) = (p1.1.min(p2.1), p1.1.max(p2.1));
        return (min_y..=max_y).contains(&p.1);
    }

    false
}

fn is_point_on_any_edge(p: &Point, edges: &Edges) -> bool {
    for edge in edges {
        if is_point_on_edge(p, edge) {
            // println!("{:?} hit edge {:?}", p, edge);
            return true;
        }
    }

    false
}

fn is_inside(p: &Point, points: &Points) -> bool {
    // try to go right from the point, keeping track of how
    // many edges we hit. #edges_hit is even, we're outside
    // otherwise, we're inside.
    // Practically, we go all the way till the max x coordinate

    if points.contains(p) {
        return true;
    }

    let edges = get_vertical_edges(points);

    if is_point_on_any_edge(p, &edges) {
        return true;
    }

    let max_x = edges.iter().map(|(p1, p2)| p1.0.max(p2.0)).max().unwrap();
    let mut inside = false;

    for xi in p.0..=max_x {
        if is_point_on_any_edge(&(xi, p.1), &edges) {
            inside = !inside;
        }
    }

    inside
}

fn display_edges(edges: &Edges) {
    if edges.is_empty() {
        return;
    }

    // Collect all vertices
    let vertices: HashSet<Point> = edges.iter().flat_map(|(p1, p2)| [**p1, **p2]).collect();

    // Collect all edge points
    let mut edge_points = HashSet::new();
    for (p1, p2) in edges {
        // Add all points between p1 and p2
        let (x1, y1) = **p1;
        let (x2, y2) = **p2;

        if x1 == x2 {
            // Vertical edge
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            for y in min_y..=max_y {
                edge_points.insert((x1, y));
            }
        } else if y1 == y2 {
            // Horizontal edge
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            for x in min_x..=max_x {
                edge_points.insert((x, y1));
            }
        }
    }

    let max_x = vertices.iter().map(|(x, _)| x).max().unwrap();
    let max_y = vertices.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if vertices.contains(&(x, y)) {
                print!("#");
            } else if edge_points.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn p2(input: &str) -> usize {
    let points = parse(input);

    let t = points
        .iter()
        .tuple_combinations()
        .filter(|((x1, y1), (x2, y2))| {
            let p3 = (*x1, *y2);
            let p4 = (*x2, *y1);

            is_inside(&p3, &points) && is_inside(&p4, &points)
        })
        .map(|(r1, r2)| (r1, r2, area(r1, r2)))
        .max_by_key(|x| x.2);
    dbg!(t);
    // dbg!(&points);
    // dbg!(is_inside(&(7, 7), &points));

    // for (p1, p2) in points.iter().tuple_combinations() {
    //     let (x1, y1) = p1;
    //     let (x2, y2) = p2;
    //     let p3 = (*x1, *y2);
    //     let p4 = (*x2, *y1);
    //
    //     if *p1 == (11, 7) && *p2 == (7, 1) {
    //         println!("{:?}, {:?}, {:?}, {:?} are inside", p1, p2, p3, p4);
    //
    //         dbg!(area(p1, p2));
    //     }
    //     // if *p1 == (9, 5) && *p2 == (2, 3) {
    //     //     println!("{:?}, {:?}, {:?}, {:?} are inside", p1, p2, p3, p4);
    //     //
    //     //     dbg!(area(p1, p2));
    //     // }
    //
    //     if is_inside(&p3, &points) && is_inside(&p4, &points) {
    //         // println!("{:?}, {:?}, {:?}, {:?} are inside", p1, p2, p3, p4);
    //     }
    // }

    todo!()
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
        assert_eq!(p1(EXAMPLE), 50);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 24);
    }
}

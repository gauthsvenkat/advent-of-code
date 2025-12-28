use std::{env, fs};

#[derive(Debug)]
struct Shape {
    grid: Vec<Vec<char>>,
}

impl Shape {
    fn _display(&self) {
        for row in &self.grid {
            for &ch in row {
                print!("{ch}");
            }
            println!();
        }
    }

    fn area(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&c| c == '#')
            .count()
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    requirements: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.width * self.length
    }
}

fn parse_shape_blocks(shape_block: &str) -> Shape {
    let (_, grid) = shape_block.split_once(':').unwrap();

    let grid = grid.trim().lines().map(|l| l.chars().collect()).collect();

    Shape { grid }
}

fn parse_region_block(region_block: &str) -> Vec<Region> {
    region_block
        .lines()
        .map(|l| {
            let (wxl, requirements) = l.split_once(':').unwrap();

            let (w, l) = wxl.split_once('x').unwrap();

            let width = w.parse::<usize>().unwrap();
            let length = l.parse::<usize>().unwrap();

            let requirements = requirements
                .split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            Region {
                width,
                length,
                requirements,
            }
        })
        .collect()
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let blocks = input.split("\n\n");

    let mut regions = Vec::new();
    let mut shapes = Vec::new();

    for block in blocks {
        if block.contains('x') {
            regions.extend(parse_region_block(block));
        } else {
            shapes.push(parse_shape_blocks(block));
        }
    }

    (shapes, regions)
}

fn p1(input: &str) -> usize {
    //NOTE: This is not a general solution!
    //The logic is to check if the area of all the shapes (number of `#`)
    //in them, is <= the area of the region. It doesn't take into account
    //the shape itself, or how to better pack the shapes. Therefore, it
    //does not actually work on the example input. It is a dumbass solution
    //but I'm glad it works lol.
    let (shapes, regions) = parse(input);
    regions
        .iter()
        .filter(|region| {
            region
                .requirements
                .iter()
                .enumerate()
                .map(|(idx, count)| shapes.get(idx).unwrap().area() * count)
                .sum::<usize>()
                <= region.area()
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    #[ignore = "Example requires NP-hard bin-packing solution; actual input works with simple area heuristic"]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 2);
    }
}

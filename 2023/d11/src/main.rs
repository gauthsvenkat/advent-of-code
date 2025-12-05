use std::collections::HashSet;
use std::env;
use std::fs;

type Image = Vec<Vec<char>>;
type Point = (usize, usize);

fn parse(input: &str) -> Image {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn find_galaxies(image: &Image) -> Vec<Point> {
    image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &c)| {
                    if c == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn empty_rows_and_cols(image: &Image) -> (HashSet<usize>, HashSet<usize>) {
    let (num_rows, num_cols) = (image.len(), image[0].len());
    let rows = image[0..num_rows]
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|&c| c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let cols = (0..num_cols)
        .filter(|&j| image.iter().all(|row| row[j] == '.'))
        .collect();

    (rows, cols)
}

fn distance(
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
    a: &Point,
    b: &Point,
    factor: usize,
) -> usize {
    fn calc_distance(empty: &HashSet<usize>, a: usize, b: usize, factor: usize) -> usize {
        let (a, b) = if a > b { (b, a) } else { (a, b) };

        (a..b)
            .map(|i| if empty.contains(&i) { factor } else { 1 })
            .sum()
    }

    calc_distance(empty_rows, a.0, b.0, factor) + calc_distance(empty_cols, a.1, b.1, factor)
}

fn p1(input: &str) -> usize {
    let image = parse(input);
    let galaxy_locations = find_galaxies(&image);
    let num_galaxies = galaxy_locations.len();

    let (empty_rows, empty_cols) = empty_rows_and_cols(&image);

    let mut r = 0;

    for i in 0..num_galaxies - 1 {
        for j in i + 1..num_galaxies {
            r += distance(
                &empty_rows,
                &empty_cols,
                &galaxy_locations[i],
                &galaxy_locations[j],
                2,
            );
        }
    }

    r
}

fn p2(input: &str) -> usize {
    let image = parse(input);
    let galaxy_locations = find_galaxies(&image);
    let num_galaxies = galaxy_locations.len();

    let (empty_rows, empty_cols) = empty_rows_and_cols(&image);

    let mut r = 0;

    for i in 0..num_galaxies - 1 {
        for j in i + 1..num_galaxies {
            r += distance(
                &empty_rows,
                &empty_cols,
                &galaxy_locations[i],
                &galaxy_locations[j],
                1000000,
            );
        }
    }

    r
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
        assert_eq!(p1(EXAMPLE), 374);
    }

    // Note: p2 uses expansion factor of 1000000 for the actual puzzle,
    // but the example would need factor 100 to get answer 8410.
    // Since p2 hardcodes the factor, we can't directly test it with the example.
}

use std::env;
use std::fs;

fn parse(input: &str) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = input.lines().collect();

    let parser = |l: &str| -> Vec<usize> {
        l.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    };

    let times = if let Some((_, times)) = lines[0].split_once(':') {
        parser(times)
    } else {
        panic!("Invalid input")
    };

    let distances = if let Some((_, distances)) = lines[1].split_once(':') {
        parser(distances)
    } else {
        panic!("Invalid input")
    };

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect()
}

fn num_ways(time: usize, distance: usize) -> usize {
    // hold_time ^ 2 - time*hold_time + distance = 0

    let (time, distance) = (time as f64, distance as f64);

    let discriminant = time.powf(2.0) - 4.0 * distance;
    let sqrt_discriminant = discriminant.sqrt();

    let s1 = (time - sqrt_discriminant) / 2.0;
    let s2 = (time + sqrt_discriminant) / 2.0;

    let s1 = if s1 == s1.trunc() {
        s1 + 1.0
    } else {
        s1.ceil()
    } as usize;

    let s2 = if s2 == s2.trunc() {
        s2 - 1.0
    } else {
        s2.floor()
    } as usize;

    s2 - s1 + 1
}

fn p1(input: &str) -> usize {
    let input = parse(input);

    input.iter().map(|&(t, d)| num_ways(t, d)).product()
}

fn p2(input: &str) -> usize {
    let input = parse(input);

    let combine = |v: Vec<usize>| -> usize {
        v.iter()
            .map(|&x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap()
    };

    let time: usize = combine(input.iter().map(|&(t, _)| t).collect());
    let distance: usize = combine(input.iter().map(|&(_, d)| d).collect());

    num_ways(time, distance)
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
        assert_eq!(p1(EXAMPLE), 288);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 71503);
    }
}

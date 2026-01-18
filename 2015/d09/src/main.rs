use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type DistanceMap<'a> = HashMap<(&'a str, &'a str), usize>;

fn parse<'a>(input: &'a str) -> (HashSet<&'a str>, DistanceMap<'a>) {
    let mut places = HashSet::new();
    let mut distance_map = HashMap::new();

    for line in input.lines() {
        let (a, rest) = line.split_once(" to ").unwrap();
        let (b, dist) = rest.split_once(" = ").unwrap();

        let dist: usize = dist.parse().unwrap();

        distance_map.insert(if a < b { (a, b) } else { (b, a) }, dist);
        places.insert(a);
        places.insert(b);
    }

    (places, distance_map)
}

fn travel(
    shortest: bool,
    from: Option<&str>,
    to: Vec<&str>,
    distance_map: &DistanceMap,
    acc: usize,
) -> usize {
    if to.is_empty() {
        return acc;
    }

    let recurse = to.iter().map(|next| {
        let dist = if let Some(from) = from {
            let key = if from < *next {
                (from, *next)
            } else {
                (*next, from)
            };

            *distance_map.get(&key).unwrap()
        } else {
            0
        };

        let to_visit: Vec<&str> = to.iter().filter(|&&n| n != *next).copied().collect();

        travel(shortest, Some(next), to_visit, distance_map, acc + dist)
    });

    (if shortest {
        recurse.min()
    } else {
        recurse.max()
    })
    .unwrap()
}

fn p1(input: &str) -> usize {
    let (places, distance_map) = parse(input);
    travel(
        true,
        None,
        places.iter().copied().collect(),
        &distance_map,
        0,
    )
}

fn p2(input: &str) -> usize {
    let (places, distance_map) = parse(input);
    travel(
        false,
        None,
        places.iter().copied().collect(),
        &distance_map,
        0,
    )
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
        assert_eq!(p1(EXAMPLE), 605);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 982);
    }
}

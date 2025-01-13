use std::collections::HashMap;
use std::env;
use std::fs;

type Map = Vec<(usize, usize, usize)>;
type Maps = HashMap<String, Map>;
type Range = (usize, usize);
type Ranges = Vec<Range>;

fn parse(input: &str) -> (Vec<usize>, Maps) {
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<usize> = if let Some((_, seeds)) = blocks[0].split_once(':') {
        seeds
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    } else {
        panic!("Invalid input");
    };

    let mut maps = HashMap::new();

    for block in blocks.iter().skip(1) {
        let map_name = if let Some((map_name, _)) = block.split_once(':') {
            map_name.replace(" map", "")
        } else {
            panic!("Invalid input");
        };

        maps.insert(
            map_name,
            block
                .lines()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect()
                })
                .map(|v: Vec<usize>| (v[0], v[1], v[2]))
                .collect::<Map>(),
        );
    }

    (seeds, maps)
}

fn get(maps: &Maps, map_type: &str, n: usize) -> usize {
    for (d, s, l) in maps.get(map_type).unwrap() {
        if n >= *s && n < s + l {
            return n - s + d;
        }
    }

    n
}

fn mapper(maps: &Maps, map_type: &str, mut inputs: Ranges, mut outputs: Ranges) -> Ranges {
    while let Some((is, ir)) = inputs.pop() {
        let ie = is + (ir - 1);

        for (od, os, or) in maps.get(map_type).unwrap() {
            let oe = os + (or - 1);
            // If the input range is contained
            // in the output range
            if is >= *os && ie < oe {
                outputs.push((is - os + od, *or))
            } else if is < *os && ie < oe {
                todo!()
            } else if is >= *os && ie >= oe {
                todo!()
            } else {
                todo!()
            }
        }
    }

    outputs
}

fn p1(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    seeds
        .iter()
        .map(|&n| get(&maps, "seed-to-soil", n))
        .map(|n| get(&maps, "soil-to-fertilizer", n))
        .map(|n| get(&maps, "fertilizer-to-water", n))
        .map(|n| get(&maps, "water-to-light", n))
        .map(|n| get(&maps, "light-to-temperature", n))
        .map(|n| get(&maps, "temperature-to-humidity", n))
        .map(|n| get(&maps, "humidity-to-location", n))
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    seeds
        .chunks(2)
        .flat_map(|s| s[0]..s[0] + s[1])
        .map(|n| get(&maps, "seed-to-soil", n))
        .map(|n| get(&maps, "soil-to-fertilizer", n))
        .map(|n| get(&maps, "fertilizer-to-water", n))
        .map(|n| get(&maps, "water-to-light", n))
        .map(|n| get(&maps, "light-to-temperature", n))
        .map(|n| get(&maps, "temperature-to-humidity", n))
        .map(|n| get(&maps, "humidity-to-location", n))
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

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

fn map_range(map: &Map, (input_start, input_range): Range) -> Ranges {
    let input_end = input_start + input_range;

    for (destination_start, source_start, range) in map {
        let source_end = source_start + range;

        // Cases to handle
        // 1) Input range entire outside ALL source ranges.
        // 2) Input range entirely inside some source range.
        // 3) Input range partially inside some source range.
        //      3.1) Input starts inside some source range, ends outside some source range.
        //      3.2) Input starts outside some source range, ends inside some source range.
        //      3.3) Input starts outside some source range, ends outside some source range.

        if input_end <= *source_start || input_start >= source_end {
            continue;
        } else if input_start >= *source_start && input_end <= source_end {
            return vec![(destination_start + input_start - source_start, input_range)];
        } else if input_start >= *source_start && input_end > source_end {
            return map_range(map, (input_start, source_end - input_start))
                .into_iter()
                .chain(map_range(map, (source_end, input_end - source_end)))
                .collect();
        } else if input_start < *source_start && input_end <= source_end {
            return map_range(map, (input_start, source_start - input_start))
                .into_iter()
                .chain(map_range(map, (*source_start, input_end - source_start)))
                .collect();
        } else if input_start < *source_start && input_end > source_end {
            return map_range(map, (input_start, source_start - input_start))
                .into_iter()
                .chain(map_range(map, (*source_start, *range)))
                .chain(map_range(map, (source_end, input_end - source_end)))
                .collect();
        }
    }

    vec![(input_start, input_range)]
}

fn p1(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    seeds
        .iter()
        .map(|&n| map_range(&maps["seed-to-soil"], (n, 1))[0].0)
        .map(|n| map_range(&maps["soil-to-fertilizer"], (n, 1))[0].0)
        .map(|n| map_range(&maps["fertilizer-to-water"], (n, 1))[0].0)
        .map(|n| map_range(&maps["water-to-light"], (n, 1))[0].0)
        .map(|n| map_range(&maps["light-to-temperature"], (n, 1))[0].0)
        .map(|n| map_range(&maps["temperature-to-humidity"], (n, 1))[0].0)
        .map(|n| map_range(&maps["humidity-to-location"], (n, 1))[0].0)
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    seeds
        .chunks(2)
        .flat_map(|w| map_range(&maps["seed-to-soil"], (w[0], w[1])))
        .flat_map(|(n, r)| map_range(&maps["soil-to-fertilizer"], (n, r)))
        .flat_map(|(n, r)| map_range(&maps["fertilizer-to-water"], (n, r)))
        .flat_map(|(n, r)| map_range(&maps["water-to-light"], (n, r)))
        .flat_map(|(n, r)| map_range(&maps["light-to-temperature"], (n, r)))
        .flat_map(|(n, r)| map_range(&maps["temperature-to-humidity"], (n, r)))
        .flat_map(|(n, r)| map_range(&maps["humidity-to-location"], (n, r)))
        .map(|(n, _)| n)
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 35);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 46);
    }
}

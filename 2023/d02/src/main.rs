use std::collections::BTreeMap;
use std::env;
use std::fs;

fn parse(input: &str) -> BTreeMap<usize, Vec<(usize, usize, usize)>> {
    let mut games = BTreeMap::new();

    input.lines().for_each(|line| {
        if let Some((game_str, rest)) = line.split_once(':') {
            let game_id: usize = game_str.replace("Game ", "").parse().unwrap();

            let mut record = Vec::new();

            for draw in rest.split(';') {
                let mut counts = (0, 0, 0);

                for cube_str in draw.split(',') {
                    if let Some((count, color)) = cube_str.trim().split_once(' ') {
                        let count: usize = count.parse().unwrap();
                        match color {
                            "red" => counts.0 = count,
                            "green" => counts.1 = count,
                            "blue" => counts.2 = count,
                            _ => panic!("Unknown color"),
                        }
                    }
                }

                record.push(counts);
            }

            games.insert(game_id, record);
        } else {
            panic!("Couldn't parse line");
        }
    });

    games
}

fn p1(input: &str) -> usize {
    let games = parse(input);

    games
        .iter()
        .filter_map(|(gid, record)| {
            for draw in record {
                if draw.0 > 12 || draw.1 > 13 || draw.2 > 14 {
                    return None;
                }
            }

            Some(gid)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let games = parse(input);

    games
        .values()
        .map(|record| {
            let mut max_draw = (0, 0, 0);

            for draw in record {
                if draw.0 > max_draw.0 {
                    max_draw.0 = draw.0;
                }
                if draw.1 > max_draw.1 {
                    max_draw.1 = draw.1;
                }
                if draw.2 > max_draw.2 {
                    max_draw.2 = draw.2;
                }
            }

            max_draw.0 * max_draw.1 * max_draw.2
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 8);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 2286);
    }
}

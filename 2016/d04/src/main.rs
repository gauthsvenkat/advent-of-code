use std::{collections::HashMap, env, fs};

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

fn parse(input: &str) -> Vec<Room> {
    input
        .lines()
        .map(|line| {
            let (name, rest) = line.rsplit_once('-').unwrap();
            let (sector_id, rest) = rest.split_once('[').unwrap();
            let checksum = rest.strip_suffix(']').unwrap();

            let name = name.chars().filter(|c| c.is_ascii_lowercase()).collect();
            let sector_id = sector_id.parse().unwrap();
            let checksum = checksum.to_string();

            Room {
                name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

fn is_real(room: &Room) -> bool {
    let mut counts: Vec<(char, usize)> = {
        let mut counts = HashMap::new();

        for ch in room.name.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        counts.into_iter().collect()
    };

    counts.sort_unstable_by(|(c1, n1), (c2, n2)| n2.cmp(n1).then(c1.cmp(c2)));

    let calculated_checksum: String = counts.iter().map(|(c, _)| c).collect();

    calculated_checksum[..5] == room.checksum[..5]
}

fn p1(input: &str) -> u32 {
    parse(input)
        .iter()
        .filter_map(|room| {
            if is_real(room) {
                Some(room.sector_id)
            } else {
                None
            }
        })
        .sum()
}

fn rotate(ch: char, n: u32) -> char {
    let offset = 'a' as u32;
    let ch = ch as u32;

    let res = (ch - offset + n) % 26;

    char::from_u32(res + offset).unwrap()
}

fn decrypt(s: &str, n: u32) -> String {
    s.chars().map(|ch| rotate(ch, n)).collect()
}

fn p2(input: &str, needle: &str) -> u32 {
    parse(input)
        .iter()
        .map(|room| {
            let id = room.sector_id;
            let name = decrypt(&room.name, id);
            (name, id)
        })
        .find_map(|(name, id)| if name == needle { Some(id) } else { None })
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = fs::read_to_string(filepath).unwrap();

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input, "northpoleobjectstorage")),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), 1514);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE, "bchofsozfcca"), 404);
    }
}

use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type Passport = HashMap<String, String>;

fn parse(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|block| {
            let fields: Vec<&str> = block
                .lines()
                .flat_map(|line| line.split_whitespace().collect::<Vec<&str>>())
                .collect();

            fields
                .iter()
                .map(|field| {
                    let (key, value) = field.split_once(':').unwrap();

                    (key.to_string(), value.to_string())
                })
                .collect()
        })
        .collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|passport| {
            passport.len() == 8 || passport.len() == 7 && !passport.contains_key("cid")
        })
        .count()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|passport| {
            // 1. Birth year
            if let Some(year) = passport.get("byr") {
                let year: usize = year.parse().unwrap();

                if !(1920..=2002).contains(&year) {
                    return false;
                }
            } else {
                return false;
            }

            // 2. Issue year
            if let Some(year) = passport.get("iyr") {
                let year: usize = year.parse().unwrap();

                if !(2010..=2020).contains(&year) {
                    return false;
                }
            } else {
                return false;
            }

            // 3. Expiration year
            if let Some(year) = passport.get("eyr") {
                let year: usize = year.parse().unwrap();

                if !(2020..=2030).contains(&year) {
                    return false;
                }
            } else {
                return false;
            }

            // 4. Height
            if let Some(height) = passport.get("hgt") {
                let unit = &height[height.len() - 2..];
                let value: usize = height[..height.len() - 2].parse().unwrap();

                match unit {
                    "cm" if !(150..=193).contains(&value) => return false,
                    "in" if !(59..=76).contains(&value) => return false,
                    "cm" | "in" => (),
                    _ => return false,
                }
            } else {
                return false;
            }

            // 5. Hair color
            if let Some(color) = passport.get("hcl") {
                if color.len() != 7 || !color.starts_with('#') {
                    return false;
                }

                for c in color[1..].chars() {
                    match c {
                        '0'..='9' => (),
                        'a'..='f' => (),
                        _ => return false,
                    }
                }
            } else {
                return false;
            }

            // 6. Eye color
            if let Some(color) = passport.get("ecl") {
                let valid_colors = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
                if !valid_colors.contains(color.as_str()) {
                    return false;
                }
            } else {
                return false;
            }

            // 7. Passport ID
            if let Some(id) = passport.get("pid") {
                if id.len() != 9 {
                    return false;
                }
            } else {
                return false;
            }

            true
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
        "p2" => println!("{}", p2(&input)),
        _ => panic!("Invalid part"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");
    const EXAMPLE3: &str = include_str!("../eg3.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 2);
        assert_eq!(p1(EXAMPLE2), 4);
        assert_eq!(p1(EXAMPLE3), 4);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 2);
        assert_eq!(p2(EXAMPLE2), 0);
        assert_eq!(p2(EXAMPLE3), 4);
    }
}

use itertools::Itertools;
use md5::{Digest, Md5};
use rayon::prelude::*;
use std::{collections::HashSet, env, fs, sync::Mutex};

fn parse(input: &str) -> String {
    input.to_string()
}

fn md5(key: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(key);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn maybe_get_chars(key: &str, prefix: &str, maybe_ans: usize) -> Option<(char, char)> {
    let key = format!("{key}{maybe_ans}");
    let res = md5(&key);

    let upto = prefix.len();

    if &res[..upto] == prefix {
        Some((
            res.chars().nth(upto).unwrap(),
            res.chars().nth(upto + 1).unwrap(),
        ))
    } else {
        None
    }
}

fn p1(input: &str) -> String {
    let key = parse(input);

    (0..usize::MAX)
        .par_bridge()
        .filter_map(|n| maybe_get_chars(&key, "00000", n).map(|(c, _)| (n, c)))
        .take_any(8)
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by_key(|(n, _)| *n)
        .map(|(_, c)| c)
        .collect()
}

fn p2(input: &str) -> String {
    let key = parse(input);

    let needed = Mutex::new(HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7']));

    (0..usize::MAX)
        .par_bridge()
        .filter_map(|n| {
            if let Some((pos, c)) = maybe_get_chars(&key, "00000", n) {
                let mut needed = needed.lock().unwrap();
                if needed.contains(&pos) {
                    needed.remove(&pos);
                    Some((pos, c))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .take_any(8)
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by_key(|(n, _)| *n)
        .map(|(_, c)| c)
        .collect()
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
    #[ignore = "Bruteforce solution. Disabling test to save compute."]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), "18f47a30");
    }

    #[test]
    #[ignore = "Bruteforce solution. Disabling test to save compute."]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), "05ace8e3");
    }
}

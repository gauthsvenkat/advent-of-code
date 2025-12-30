use md5::{Digest, Md5};
use rayon::prelude::*;
use std::{env, fs};

fn parse(input: &str) -> String {
    input.to_string()
}

fn md5(key: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(key);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn try_answer(key: &str, prefix: &str, maybe_ans: usize) -> bool {
    let key = format!("{key}{maybe_ans}");
    let res = md5(&key);

    &res[..prefix.len()] == prefix
}

fn p1(input: &str) -> usize {
    let key = parse(input);

    (0..usize::MAX)
        .par_bridge()
        .find_any(|n| try_answer(&key, "00000", *n))
        .unwrap()
}

fn p2(input: &str) -> usize {
    let key = parse(input);

    (0..usize::MAX)
        .par_bridge()
        .find_any(|n| try_answer(&key, "000000", *n))
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

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");

    #[test]
    #[ignore = "Solution just bruteforces in parallel. Disabling test to save compute."]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 609043);
        assert_eq!(p1(EXAMPLE2), 1048970);
    }

    #[test]
    #[ignore = "Solution just bruteforces in parallel. Disabling test to save compute."]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE1), 6742839);
        assert_eq!(p2(EXAMPLE2), 5714438);
    }
}

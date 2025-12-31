use std::{env, fs};

#[derive(Debug)]
struct IP {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

fn extract_hypernet_sequence(sequence: &mut String) -> Vec<String> {
    let mut extracted = Vec::new();

    while let Some(s) = sequence.find('[') {
        if let Some(e) = sequence[s..].find(']') {
            let e = s + e;

            extracted.push(sequence[s + 1..e].to_string());
            sequence.replace_range(s..=e, " ");
        }
    }

    extracted
}

fn parse(input: &str) -> Vec<IP> {
    input
        .lines()
        .map(|line| {
            let mut sequences = line.to_string();

            let hypernets = extract_hypernet_sequence(&mut sequences);

            let supernets = sequences
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            IP {
                supernets,
                hypernets,
            }
        })
        .collect()
}

fn is_abba(s: &[char]) -> bool {
    if s[0] == s[1] {
        // pair should be different
        false
    } else {
        // pair should be mirrored
        s[0] == s[3] && s[1] == s[2]
    }
}
fn is_abba_seq(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();

    s.windows(4).any(is_abba)
}
fn is_ip_abba(ip: &IP) -> bool {
    let has_abba_seq = ip.supernets.iter().any(|s| is_abba_seq(s));
    let has_abba_hypernet = ip.hypernets.iter().any(|s| is_abba_seq(s));

    has_abba_seq && !has_abba_hypernet
}

fn p1(input: &str) -> usize {
    parse(input).iter().filter(|ip| is_ip_abba(ip)).count()
}

fn get_abas(s: &str) -> Vec<(char, char, char)> {
    let s: Vec<char> = s.chars().collect();

    let mut abas = Vec::new();

    for w in s.windows(3) {
        if w[0] == w[2] && w[0] != w[1] {
            abas.push((w[0], w[1], w[2]));
        }
    }

    abas
}

fn get_abas_from_ip(ip: &IP) -> Vec<(char, char, char)> {
    let mut abas = Vec::new();

    for s in ip.supernets.iter() {
        abas.extend(get_abas(s));
    }

    abas
}

fn supports_ssl(ip: &IP) -> bool {
    let abas = get_abas_from_ip(ip);

    for (a, b, _) in abas {
        let expected_bab = format!("{b}{a}{b}");

        if ip.hypernets.iter().any(|hn| hn.contains(&expected_bab)) {
            return true;
        }
    }

    false
}

fn p2(input: &str) -> usize {
    parse(input).iter().filter(|ip| supports_ssl(ip)).count()
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
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), 3);
    }
}

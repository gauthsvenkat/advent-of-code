use cached::{proc_macro::cached, UnboundCache};
use std::env;
use std::fs;

struct Springs {
    record: String,
    sizes: Vec<usize>,
}

fn parse(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|l| {
            if let Some((record, sizes)) = l.split_once(' ') {
                let record = record.trim().to_string();
                let sizes = sizes
                    .trim()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();

                Springs { record, sizes }
            } else {
                panic!("Couldn't parse line!");
            }
        })
        .collect()
}

#[cached(
    ty = "UnboundCache<String, usize>",
    create = "{UnboundCache::new()}",
    convert = r#"{format!("{}{:?}", record, sizes)}"#
)]
fn count(record: &str, sizes: &[usize]) -> usize {
    if record.is_empty() {
        return if sizes.is_empty() { 1 } else { 0 };
    } else if sizes.is_empty() {
        return if record.contains('#') { 0 } else { 1 };
    // If the length of the string is less than the
    // number of expected '#' + the inbetween '.' characters
    // then we can't have a valid record
    } else if record.len() < sizes.iter().sum::<usize>() + sizes.len() - 1 {
        return 0;
    }

    let mut counts = 0;

    if record.starts_with('.') || record.starts_with('?') {
        counts += count(&record[1..], sizes);
    }

    if record.starts_with('#') || record.starts_with('?') {
        let n = sizes[0];
        let (block, rest) = record.split_at(n);

        if block.chars().all(|c| c != '.') {
            if rest.is_empty() {
                counts += count(rest, &sizes[1..]);
            } else if !rest.starts_with('#') {
                counts += count(&rest[1..], &sizes[1..]);
            }
        }
    }

    counts
}

fn p1(input: &str) -> usize {
    let spring_field = parse(input);
    spring_field
        .iter()
        .map(|s| count(&s.record, &s.sizes))
        .sum()
}

fn p2(input: &str) -> usize {
    let spring_field = parse(input);
    spring_field
        .iter()
        .map(
            |Springs {
                 record: r,
                 sizes: s,
             }| Springs {
                record: format!("{}?{}?{}?{}?{}", r, r, r, r, r),
                sizes: s.iter().cycle().take(s.len() * 5).copied().collect(),
            },
        )
        .map(|s| count(&s.record, &s.sizes))
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

use std::collections::HashMap;
use std::env;
use std::fs;

fn readfile(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}

fn parse(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    // Key is the page number and the values are the pages that are supposed
    // to be after the key.
    let mut orderings: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
            orderings.entry(parts[0]).or_default().push(parts[1]);
        } else if line.contains(',') {
            let nums: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(nums);
        }
    }

    (orderings, updates)
}

fn check(update: &Vec<i32>, orderings: &HashMap<i32, Vec<i32>>) -> bool {
    let update_len = update.len();

    for (i, &current_page) in update.iter().enumerate().take(update_len - 1) {
        for &next_page in update.iter().skip(i + 1) {
            if let Some(pages) = orderings.get(&next_page) {
                if pages.contains(&current_page) {
                    return false;
                }
            }
        }
    }
    true
}

fn p1(input: &str) -> i32 {
    let (orderings, updates) = parse(input);

    updates
        .iter()
        .filter(|&v| check(v, &orderings))
        .map(|v| v[v.len() / 2])
        .sum()
}

fn fix(mut update: Vec<i32>, orderings: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let update_len = update.len();

    let mut m: Option<(usize, usize)> = None;

    'outer: for (i, &current_page) in update.iter().enumerate().take(update_len - 1) {
        for (j, &next_page) in update.iter().enumerate().skip(i + 1) {
            if let Some(pages) = orderings.get(&next_page) {
                if pages.contains(&current_page) {
                    m = Some((i, j));
                    break 'outer;
                }
            }
        }
    }

    match m {
        Some((i, j)) => {
            update.swap(i, j);
            fix(update, orderings)
        }
        None => update,
    }
}

fn p2(input: &str) -> i32 {
    let (orderings, updates) = parse(input);

    let incorrect_updates: Vec<Vec<i32>> = updates
        .iter()
        .filter(|v| !check(v, &orderings))
        .cloned()
        .collect();

    let fixed_updates: Vec<Vec<i32>> = incorrect_updates
        .iter()
        .map(|v| fix(v.clone(), &orderings))
        .collect();

    fixed_updates.iter().map(|v| v[v.len() / 2]).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    let filepath = &args[2];

    let input = readfile(filepath);

    match part.as_str() {
        "p1" => println!("{}", p1(&input)),
        "p2" => println!("{}", p2(&input)),
        _ => panic!("Invalid part"),
    };
}

use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::RangeInclusive;

type IDRange = RangeInclusive<usize>;

fn parse(input: &str) -> (HashSet<IDRange>, Vec<usize>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let id_ranges = parts[0]
        .lines()
        .map(|l| {
            let nums: Vec<usize> = l.split('-').map(|n| n.parse().unwrap()).collect();
            nums[0]..=nums[1]
        })
        .collect();

    let ingredients = parts[1].lines().map(|l| l.parse().unwrap()).collect();

    (id_ranges, ingredients)
}

fn p1(input: &str) -> usize {
    let (id_ranges, ingredients) = parse(input);

    ingredients
        .iter()
        .filter(|i| id_ranges.iter().any(|r| r.contains(i)))
        .count()
}

enum Overlap<'a> {
    None,
    Full(&'a IDRange),
    Partial(IDRange),
}

fn is_overlapping<'a>(a: &'a IDRange, b: &'a IDRange) -> Overlap<'a> {
    // Assumption; (a.start, b.end) < (b.start, a.end)

    if !a.contains(b.start()) && !a.contains(b.end()) {
        Overlap::None
    } else if a.contains(b.start()) && a.contains(b.end()) {
        Overlap::Full(b)
    } else if a.contains(b.start()) && !a.contains(b.end()) {
        Overlap::Partial(*b.start()..=*a.end())
    } else {
        unreachable!("Should have determined overlappiness by now")
    }
}

enum OverlapAction {
    NoAction,
    Replace {
        remove: HashSet<IDRange>,
        add: HashSet<IDRange>,
    },
}

fn handle_overlap(a: &IDRange, b: &IDRange) -> OverlapAction {
    // switching a and b the following way allows us to collapse
    // `is_overlapping` to a few cases, that are symmetric.
    let (a, b) = if (a.start(), b.end()) < (b.start(), a.end()) {
        (a, b)
    } else {
        (b, a)
    };

    match is_overlapping(a, b) {
        Overlap::None => OverlapAction::NoAction,
        // b is fully contained in a
        Overlap::Full(b) => OverlapAction::Replace {
            remove: HashSet::from([b.to_owned()]),
            add: HashSet::from([a.to_owned()]),
        },
        Overlap::Partial(c) => OverlapAction::Replace {
            remove: HashSet::from([a.to_owned(), b.to_owned()]),
            add: HashSet::from([*a.start()..=(c.start() - 1), *c.end() + 1..=*b.end(), c]),
        },
    }
}

fn next_action(id_ranges: &HashSet<IDRange>) -> OverlapAction {
    // Go over all pairs, find the first pair we need to act upon, and return the action
    for (i, r1) in id_ranges.iter().enumerate() {
        for r2 in id_ranges.iter().skip(i + 1) {
            let action = handle_overlap(r1, r2);

            match action {
                OverlapAction::NoAction => continue,
                _ => return action,
            }
        }
    }

    OverlapAction::NoAction
}

fn mk_disjoint_ranges(mut id_ranges: HashSet<IDRange>) -> HashSet<IDRange> {
    match next_action(&id_ranges) {
        OverlapAction::NoAction => return id_ranges,
        OverlapAction::Replace { remove, add } => {
            for r in remove.iter() {
                id_ranges.remove(r);
            }
            for r in add.into_iter() {
                id_ranges.insert(r);
            }
        }
    }

    mk_disjoint_ranges(id_ranges)
}

fn p2(input: &str) -> usize {
    let (id_ranges, _) = parse(input);

    mk_disjoint_ranges(id_ranges)
        .iter()
        .map(|r| r.end() - r.start() + 1)
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

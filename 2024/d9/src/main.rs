use std::env;
use std::fs;

fn readfile(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}

fn parse(input: &str) -> String {
    input.trim().to_string()
}

fn parse_diskmap(diskmap: &str) -> Vec<Option<usize>> {
    let mut is_freespace: bool = false;
    let mut diskmap_vec = Vec::new();
    let mut curr_idx = 0;

    for c in diskmap.chars() {
        let size: u32 = c.to_digit(10).unwrap();

        for _ in 0..size {
            diskmap_vec.push(if is_freespace { None } else { Some(curr_idx) });
        }

        if !is_freespace {
            curr_idx += 1;
        }

        is_freespace = !is_freespace;
    }

    diskmap_vec
}

fn defragment_blocks(expanded_diskmap: Vec<Option<usize>>) -> Vec<usize> {
    let mut right_idx = expanded_diskmap.len() - 1;

    let mut defragmented_diskmap = Vec::new();

    for i in 0..expanded_diskmap.len() {
        if i > right_idx {
            break;
        }

        match expanded_diskmap[i] {
            Some(a) => defragmented_diskmap.push(a),
            None => {
                while expanded_diskmap[right_idx].is_none() {
                    right_idx -= 1;
                }
                defragmented_diskmap.push(expanded_diskmap[right_idx].unwrap());
                right_idx -= 1;
            }
        }
    }

    defragmented_diskmap
}

fn p1(input: &str) -> usize {
    let diskmap = parse(input);
    let expanded_diskmap = parse_diskmap(&diskmap);
    let defragmented_diskmap = defragment_blocks(expanded_diskmap);

    defragmented_diskmap
        .iter()
        .enumerate()
        .map(|(i, a)| i * a)
        .sum()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    // TODO:
    todo!()
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

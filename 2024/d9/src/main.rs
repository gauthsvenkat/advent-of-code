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

fn checksum(expanded_diskmap: &Vec<Option<usize>>) -> usize {
    let mut sum = 0;

    let mut left_idx = 0;
    let mut right_idx = expanded_diskmap.len() - 1;

    loop {
        if left_idx > right_idx {
            break;
        }

        match expanded_diskmap[left_idx] {
            Some(a) => {
                sum += left_idx * a;
            }
            None => {
                while expanded_diskmap[right_idx].is_none() {
                    right_idx -= 1;
                }
                sum += left_idx * expanded_diskmap[right_idx].unwrap();
                right_idx -= 1;
            }
        }
        left_idx += 1;
    }

    sum
}

fn p1(input: &str) -> usize {
    let diskmap = parse(input);
    let expanded_diskmap = parse_diskmap(&diskmap);

    checksum(&expanded_diskmap)

    // for c in expanded_diskmap {
    //     match c {
    //         Some(a) => print!("{a}"),
    //         None => print!("."),
    //     }
    // }
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

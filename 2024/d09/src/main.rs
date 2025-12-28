use std::{env, fs};

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

    let mut i = 0;
    while i <= right_idx {
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
        i += 1;
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

fn defragment_files(mut expanded_diskmap: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let total_length = expanded_diskmap.len();
    let mut right_idx = total_length - 1;

    while right_idx > 0 {
        // move to start of non empty space from right
        while expanded_diskmap[right_idx].is_none() {
            right_idx -= 1;
        }

        let file_id = expanded_diskmap[right_idx].unwrap();
        let mut file_size = 0;

        while right_idx > file_size
            && match expanded_diskmap[right_idx - file_size] {
                Some(a) => a == file_id,
                None => false,
            }
        {
            file_size += 1;
        }

        let mut left_idx = 0;
        let mut empty_size = 0;

        while left_idx < right_idx - file_size {
            // move to the start of an empty space from left
            while expanded_diskmap[left_idx].is_some() {
                left_idx += 1;
            }

            // check from the left for the first empty space
            // which is >= file_size
            while expanded_diskmap[left_idx + empty_size].is_none() {
                empty_size += 1;
                if left_idx + empty_size >= total_length {
                    break;
                }
            }

            if empty_size >= file_size {
                break;
            } else {
                left_idx += empty_size;
                empty_size = 0;
            }
        }

        // If found, move the file to the empty space
        if empty_size >= file_size && left_idx < right_idx {
            // everything from right_idx to right_idx - file_size should move
            // to left_idx to left_idx + file_size

            for i in 0..file_size {
                expanded_diskmap[left_idx + i] = expanded_diskmap[right_idx - i];
                expanded_diskmap[right_idx - i] = None;
            }
        }

        right_idx -= file_size;
    }

    expanded_diskmap
}

fn p2(input: &str) -> usize {
    let diskmap = parse(input);
    let expanded_diskmap = parse_diskmap(&diskmap);
    let defragmented_diskmap = defragment_files(expanded_diskmap);

    defragmented_diskmap
        .iter()
        .enumerate()
        .map(|(i, a)| match a {
            Some(v) => i * v,
            None => 0,
        })
        .sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 1928);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 2858);
    }
}

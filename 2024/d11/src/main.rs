use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn split_stone_if_even(num: usize) -> Option<(usize, usize)> {
    let mut temp = num;
    let mut num_digits = 0;

    while temp > 0 {
        temp /= 10;
        num_digits += 1;
    }

    if num_digits % 2 != 0 {
        return None;
    }

    let half = num_digits / 2;

    let divisor = 10usize.pow(half as u32);

    Some((num / divisor, num % divisor))
}

// fn blink(arrangement: &[usize]) -> Vec<usize> {
//     let mut new_arrangement: Vec<usize> = Vec::new();
//
//     for &stone in arrangement.iter() {
//         if stone == 0 {
//             new_arrangement.push(1);
//         } else if let Some((first, second)) = split_stone_if_even(stone) {
//             new_arrangement.push(first);
//             new_arrangement.push(second);
//         } else {
//             new_arrangement.push(stone * 2024);
//         }
//     }
//
//     new_arrangement
// }
fn blink(stone: usize, mut record: HashMap<usize, (usize)>) -> HashMap<usize, usize> {
    if let Some(&value) = record.get(&stone) {
        return (value, record);
    }

    if stone == 0 {
        record.insert(stone, 1);
        return (1, record);
    } else if let Some((first, second)) = split_stone_if_even(stone) {
        record.insert(stone, 2);
        return (2, record);
    } else {
        record.insert(stone, 1);
        return (stone * 2024, record);
    }

}

fn p1(input: &str) -> usize {
    let arrangement = parse(input);
    let mut total_length = 0;

    for (i, &stone) in arrangement.iter().enumerate() {
        println!("Stone {}: {}", i, stone);
        let mut stones = vec![stone];
        for j in 0..25 {
            println!("----Blink {}", j + 1);
            stones = blink(&stones);
        }

        total_length += stones.len();
    }

    total_length
}

fn p2(input: &str) -> usize {
    let arrangement = parse(input);
    let mut total_length = 0;

    for (i, &stone) in arrangement.iter().enumerate() {
        println!("Stone {}: {}", i, stone);
        let mut stones = vec![stone];
        for j in 0..75 {
            println!("----Blink {}", j + 1);
            stones = blink(&stones);
        }

        total_length += stones.len();
    }

    total_length
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

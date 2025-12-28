use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env, fs, iter,
};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

type Position = (usize, usize);

struct Keypad {
    keys: HashMap<char, Position>,
    n_row: usize,
    n_col: usize,
    nogo: Position,
}

impl Keypad {
    fn numeric() -> Self {
        /*
        7 8 9
        4 5 6
        1 2 3
          0 A
        */
        Keypad {
            keys: HashMap::from([
                ('7', (0, 0)),
                ('8', (0, 1)),
                ('9', (0, 2)),
                ('4', (1, 0)),
                ('5', (1, 1)),
                ('6', (1, 2)),
                ('1', (2, 0)),
                ('2', (2, 1)),
                ('3', (2, 2)),
                ('0', (3, 1)),
                ('A', (3, 2)),
            ]),
            n_row: 4,
            n_col: 3,
            nogo: (3, 0),
        }
    }

    fn directional() -> Self {
        /*
          ^ A
        < v >
        */
        Keypad {
            keys: HashMap::from([
                ('^', (0, 1)),
                ('A', (0, 2)),
                ('<', (1, 0)),
                ('v', (1, 1)),
                ('>', (1, 2)),
            ]),
            n_row: 2,
            n_col: 3,
            nogo: (0, 0),
        }
    }
}

fn travel(keypad: &Keypad, current_char: char, target_char: char) -> Vec<String> {
    let (current_position, target_position) =
        (keypad.keys[&current_char], keypad.keys[&target_char]);

    let mut pq: BinaryHeap<Reverse<(usize, Position, String)>> = BinaryHeap::new();
    pq.push(Reverse((0, current_position, String::new())));

    let mut best_cost: Option<usize> = None;
    let mut paths: Vec<String> = Vec::new();

    while let Some(Reverse((cost, position, mut path))) = pq.pop() {
        let (i, j) = position;

        if position == target_position {
            if let Some(bc) = best_cost {
                if cost > bc {
                    break;
                }
            } else {
                best_cost = Some(cost);
            }

            paths.push({
                path.push('A');
                path
            });

            continue;
        }

        for (c, (di, dj)) in [('<', (0, -1)), ('v', (1, 0)), ('^', (-1, 0)), ('>', (0, 1))] {
            let (ni, nj) = (i as i8 + di, j as i8 + dj);

            if ni < 0 || nj < 0 {
                continue;
            }

            let (ni, nj) = (ni as usize, nj as usize);

            if ni >= keypad.n_row || nj >= keypad.n_col || (ni, nj) == keypad.nogo {
                continue;
            }

            pq.push(Reverse((cost + 1, (ni, nj), {
                let mut updated_path = path.clone();
                updated_path.push(c);
                updated_path
            })));
        }
    }

    paths
}

fn button_sequence(
    code: &str,
    keypad: &Keypad,
    cache: &mut HashMap<(char, char, usize), usize>,
    depth: usize,
) -> usize {
    if depth == 0 {
        return code.len();
    }

    iter::once('A')
        .chain(code.chars())
        .collect::<Vec<char>>()
        .windows(2)
        .map(|w| {
            let (current_char, next_char) = (w[0], w[1]);

            if let Some(&len) = cache.get(&(current_char, next_char, depth)) {
                len
            } else {
                let len = travel(keypad, current_char, next_char)
                    .iter()
                    .map(|path| button_sequence(path, &Keypad::directional(), cache, depth - 1))
                    .min()
                    .unwrap();

                cache.insert((current_char, next_char, depth), len);

                len
            }
        })
        .sum()
}

fn get_numeric_part(code: &str) -> usize {
    code.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn p1(input: &str) -> usize {
    let codes = parse(input);

    let mut cache = HashMap::new();

    codes
        .iter()
        .map(|code| {
            button_sequence(code, &Keypad::numeric(), &mut cache, 3) * get_numeric_part(code)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let codes = parse(input);

    let mut cache = HashMap::new();

    codes
        .iter()
        .map(|code| {
            button_sequence(code, &Keypad::numeric(), &mut cache, 26) * get_numeric_part(code)
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE), 126384);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 154115708116294);
    }
}

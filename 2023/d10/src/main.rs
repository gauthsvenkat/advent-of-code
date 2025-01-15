use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

type Position = (usize, usize);

fn get_start_location(tiles: &[Vec<char>]) -> Position {
    for (i, row) in tiles.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No starting location found");
}

fn get_starting_tile(tiles: &[Vec<char>], &(i, j): &Position) -> char {
    let (w, s, n, e) = (
        if j > 0 { tiles[i][j - 1] } else { '.' },
        if i < tiles.len() - 1 {
            tiles[i + 1][j]
        } else {
            '.'
        },
        if i > 0 { tiles[i - 1][j] } else { '.' },
        if j < tiles[0].len() - 1 {
            tiles[i][j + 1]
        } else {
            '.'
        },
    );

    match (w, s, n, e) {
        (_, 'J' | '|' | 'L', '7' | '|' | 'F', _) => '|',
        ('L' | '-' | 'F', _, _, 'J' | '-' | '7') => '-',
        ('L' | '-' | 'F', _, '7' | '|' | 'F', _) => 'J',
        (_, _, '7' | '|' | 'F', 'J' | '-' | '7') => 'L',
        (_, 'J' | '|' | 'L', _, 'J' | '-' | '7') => 'F',
        ('L' | '-' | 'F', 'J' | '|' | 'L', _, _) => '7',
        _ => panic!("Invalid starting tile"),
    }
}

fn get_adjacent_tile_locations(tile: &char, (i, j): &Position) -> (Position, Position) {
    match tile {
        '|' => ((i + 1, *j), (i - 1, *j)),
        '-' => ((*i, j - 1), (*i, j + 1)),
        'L' => ((i - 1, *j), (*i, j + 1)),
        'J' => ((i - 1, *j), (*i, j - 1)),
        '7' => ((*i, j - 1), (i + 1, *j)),
        'F' => ((i + 1, *j), (*i, j + 1)),
        _ => panic!("Invalid tile"),
    }
}

fn next_pos(pos: &Position, tile: &char, seen: &HashSet<Position>) -> Option<Position> {
    let (option1, option2) = get_adjacent_tile_locations(tile, pos);

    if !seen.contains(&option1) {
        Some(option1)
    } else if !seen.contains(&option2) {
        Some(option2)
    } else {
        None
    }
}

fn get_loop_locations(tiles: &[Vec<char>], start: &Position) -> HashSet<Position> {
    let mut seen = HashSet::new();
    let mut pos = *start;

    while let Some(next_pos) = next_pos(&pos, &tiles[pos.0][pos.1], &seen) {
        seen.insert(next_pos);
        pos = next_pos;
    }

    seen
}

fn p1(input: &str) -> usize {
    let mut tiles = parse(input);
    let start = get_start_location(&tiles);
    let start_tile = get_starting_tile(&tiles, &start);

    tiles[start.0][start.1] = start_tile;

    get_loop_locations(&tiles, &start).len() / 2
}

fn get_num_inside_tiles_in_row(row: &[char]) -> usize {
    let mut inside = false;
    let mut ridin: Option<char> = None;
    let mut count = 0;

    for c in row {
        match c {
            '|' => {
                inside = !inside;
            }
            s @ ('L' | 'F') => {
                ridin = Some(*s);
            }
            e @ ('J' | '7') => {
                match (ridin, e) {
                    (Some('L'), '7') | (Some('F'), 'J') => {
                        inside = !inside;
                    }
                    _ => continue,
                };
                ridin = None;
            }
            '.' if inside => {
                count += 1;
            }
            _ => continue,
        }
    }

    count
}

fn p2(input: &str) -> usize {
    let mut tiles = parse(input);
    let start = get_start_location(&tiles);
    let start_tile = get_starting_tile(&tiles, &start);

    tiles[start.0][start.1] = start_tile;

    let loop_locations = get_loop_locations(&tiles, &start);

    // Replace all other locations except the loop with a .
    let cleaned_tiles: Vec<Vec<char>> = tiles
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| {
                    if loop_locations.contains(&(i, j)) {
                        c
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();

    cleaned_tiles
        .iter()
        .map(|r| get_num_inside_tiles_in_row(r))
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

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_start(maze: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No start found");
}

fn step(
    maze: &[Vec<char>],
    position: (usize, usize),
    direction: char,
    mut visited: HashSet<(usize, usize)>,
    cost_map: &mut HashMap<(usize, usize), usize>,
) -> Option<usize> {
    let (i, j) = position;

    if maze[i][j] == 'E' {
        return Some(0);
    }

    if let Some(score) = cost_map.get(&position) {
        return Some(*score);
    }

    if visited.contains(&position) {
        return None;
    }

    visited.insert(position);

    let possible_dirs = match direction {
        '<' => ['<', 'v', '^'],
        'v' => ['v', '>', '<'],
        '^' => ['^', '<', '>'],
        '>' => ['>', '^', 'v'],
        _ => panic!("Invalid direction"),
    };

    possible_dirs
        .into_iter()
        .zip([1, 1001, 1001].iter())
        .filter_map(|(dir, penalty)| {
            let dir_vec = match dir {
                '<' => (0, -1),
                'v' => (1, 0),
                '^' => (-1, 0),
                '>' => (0, 1),
                _ => panic!("Invalid direction"),
            };

            let next_pos = (
                (i as i32 + dir_vec.0) as usize,
                (j as i32 + dir_vec.1) as usize,
            );

            if maze[next_pos.0][next_pos.1] == '#' {
                None
            } else if let Some(next_score) = cost_map.get(&next_pos) {
                Some(next_score + penalty)
            } else {
                step(maze, next_pos, dir, visited.clone(), cost_map)
                    .map(|next_score| next_score + penalty)
            }
        })
        .min()
        .map(|score| {
            let mut score_copy = score;
            cost_map.entry(position).and_modify(|e| *e = *e.min(&mut score_copy)).or_insert(score_copy);
            score
        })
}

fn p1(input: &str) -> usize {
    let maze = parse(input);
    let start = find_start(&maze);

    let mut cost_map = HashMap::new();

    let r = step(&maze, start, '>', HashSet::new(), &mut cost_map).unwrap_or(usize::MAX);

    dbg!(&cost_map);
    dbg!(&cost_map.get(&start));
    dbg!(&cost_map.get(&(1, 12)));

    r
}

fn p2(input: &str) -> usize {
    let parsed_input = parse(input);
    todo!()
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

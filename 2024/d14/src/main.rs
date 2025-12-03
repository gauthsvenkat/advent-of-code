use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

type Position = (u32, u32);
type Velocity = (i32, i32);

fn parse(input: &str) -> (Vec<Position>, Vec<Velocity>) {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut positions = Vec::new();
    let mut velocities = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let x = caps[1].parse::<u32>().unwrap();
        let y = caps[2].parse::<u32>().unwrap();

        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();

        positions.push((x, y));
        velocities.push((vx, vy));
    }

    (positions, velocities)
}

fn find_grid_size(positions: &[Position]) -> (u32, u32) {
    (
        positions.iter().map(|(x, _)| *x).max().unwrap() + 1,
        positions.iter().map(|(_, y)| *y).max().unwrap() + 1,
    )
}

fn find_position(
    seconds: usize,
    grid_dims: &(u32, u32),
    position: &(u32, u32),
    velocity: &(i32, i32),
) -> (u32, u32) {
    let (x, y) = position;
    let (vx, vy) = velocity;
    let (w, h) = grid_dims;

    (
        (*x as i32 + vx * (seconds as i32)).rem_euclid(*w as i32) as u32,
        (*y as i32 + vy * (seconds as i32)).rem_euclid(*h as i32) as u32,
    )
}

fn safety_factor(grid_dims: (u32, u32), positions: &[Position]) -> u32 {
    let (w, h) = grid_dims;
    let mut quads = [0; 4];

    for (x, y) in positions.iter() {
        if *x < w / 2 && *y < h / 2 {
            quads[0] += 1;
        } else if *x > w / 2 && *y < h / 2 {
            quads[1] += 1;
        } else if *x < w / 2 && *y > h / 2 {
            quads[2] += 1;
        } else if *x > w / 2 && *y > h / 2 {
            quads[3] += 1;
        }
    }

    quads[0] * quads[1] * quads[2] * quads[3]
}

fn p1(input: &str) -> u32 {
    let (positions, velocities) = parse(input);
    let grid_dims = find_grid_size(&positions);

    let latest_positions: Vec<(u32, u32)> = positions
        .iter()
        .zip(&velocities)
        .map(|(pos, vel)| find_position(100, &grid_dims, pos, vel))
        .collect();

    safety_factor(grid_dims, &latest_positions)
}

fn render_positions(grid_dims: (u32, u32), positions: &[Position]) -> String {
    let (w, h) = grid_dims;

    let mut position_counts: HashMap<(u32, u32), usize> = HashMap::new();

    for &(x, y) in positions.iter() {
        *position_counts.entry((x, y)).or_insert(0) += 1;
    }

    let mut grid = vec![vec!['.'; w as usize]; h as usize];

    for ((x, y), count) in position_counts {
        grid[y as usize][x as usize] = char::from_digit(count as u32, 10).unwrap_or('+');
    }

    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn has_full_3x3_block(rendered: &str) -> bool {
    let grid: Vec<Vec<char>> = rendered
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    for y in 0..height.saturating_sub(2) {
        for x in 0..width.saturating_sub(2) {
            if is_3x3_block_occupied(&grid, x, y) {
                return true;
            }
        }
    }

    false
}

fn is_3x3_block_occupied(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    for dy in 0..3 {
        for dx in 0..3 {
            if grid[y + dy][x + dx] == '.' {
                return false;
            }
        }
    }
    true
}

fn p2(input: &str) -> usize {
    let (positions, velocities) = parse(input);
    let grid_dims = find_grid_size(&positions);

    for second in 1000..10000 {
        let render = render_positions(
            grid_dims,
            &positions
                .iter()
                .zip(&velocities)
                .map(|(pos, vel)| find_position(second + 1, &grid_dims, pos, vel))
                .collect::<Vec<_>>(),
        );

        if has_full_3x3_block(&render) {
            println!("{}", render);
            return second + 1;
        }
    }

    0
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

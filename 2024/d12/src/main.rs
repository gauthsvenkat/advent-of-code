use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn get_contiguous(
    pos: (usize, usize),
    plot: &Vec<Vec<char>>,
    grid_dim: (usize, usize),
    mut contiguous_plot: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    if contiguous_plot.contains(&pos) {
        return contiguous_plot;
    }

    contiguous_plot.insert(pos);

    let (i, j) = pos;
    let (h, w) = grid_dim;

    if j > 0 && plot[i][j - 1] == plot[i][j] {
        contiguous_plot = get_contiguous((i, j - 1), plot, grid_dim, contiguous_plot);
    }

    if i < h - 1 && plot[i + 1][j] == plot[i][j] {
        contiguous_plot = get_contiguous((i + 1, j), plot, grid_dim, contiguous_plot);
    }

    if i > 0 && plot[i - 1][j] == plot[i][j] {
        contiguous_plot = get_contiguous((i - 1, j), plot, grid_dim, contiguous_plot);
    }

    if j < w - 1 && plot[i][j + 1] == plot[i][j] {
        contiguous_plot = get_contiguous((i, j + 1), plot, grid_dim, contiguous_plot);
    }

    contiguous_plot
}

fn count_perimeter(plot: &Vec<Vec<char>>, contiguous_plot: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;

    let max_i = plot.len() - 1;
    let max_j = plot[0].len() - 1;

    for (i, j) in contiguous_plot {
        if *j == 0 || plot[*i][j - 1] != plot[*i][*j] {
            perimeter += 1;
        }
        if *i == max_i || plot[i + 1][*j] != plot[*i][*j] {
            perimeter += 1;
        }
        if *i == 0 || plot[i - 1][*j] != plot[*i][*j] {
            perimeter += 1;
        }
        if *j == max_j || plot[*i][j + 1] != plot[*i][*j] {
            perimeter += 1;
        }
    }

    perimeter
}

fn p1(input: &str) -> usize {
    let plot = parse(input);

    let grid_dim = (plot.len(), plot[0].len());

    let mut visited_positions = HashSet::new();
    let mut contiguous_plots: Vec<HashSet<(usize, usize)>> = Vec::new();

    for i in 0..grid_dim.0 {
        for j in 0..grid_dim.1 {
            if visited_positions.contains(&(i, j)) {
                continue;
            }
            let contiguous = get_contiguous((i, j), &plot, grid_dim, HashSet::new());

            contiguous_plots.push(contiguous.clone());
            visited_positions.extend(contiguous);
        }
    }

    contiguous_plots
        .iter()
        .map(|c| count_perimeter(&plot, c) * c.len())
        .sum()
}

fn traverse_straight(
    pos: (usize, usize),
    traversal_direction: char,
    side_direction: char,
    plot: &[Vec<char>],
    grid_dim: (usize, usize),
) -> HashSet<(usize, usize, char)> {
    let (mut i, mut j) = pos;
    let (h, w) = grid_dim;

    let mut seen = HashSet::new();
    seen.insert((i, j, side_direction));

    match traversal_direction {
        'l' => match side_direction {
            'u' => {
                while j > 0
                    && plot[i][j - 1] == plot[i][j]
                    && (i == 0 || plot[i - 1][j - 1] != plot[i][j])
                {
                    j -= 1;
                    seen.insert((i, j, side_direction));
                }
            }
            'd' => {
                while j > 0
                    && plot[i][j - 1] == plot[i][j]
                    && (i == h - 1 || plot[i + 1][j - 1] != plot[i][j])
                {
                    j -= 1;
                    seen.insert((i, j, side_direction));
                }
            }
            _ => panic!("Invalid side direction"),
        },
        'd' => match side_direction {
            'l' => {
                while i < h - 1
                    && plot[i + 1][j] == plot[i][j]
                    && (j == 0 || plot[i + 1][j - 1] != plot[i][j])
                {
                    i += 1;
                    seen.insert((i, j, side_direction));
                }
            }
            'r' => {
                while i < h - 1
                    && plot[i + 1][j] == plot[i][j]
                    && (j == w - 1 || plot[i + 1][j + 1] != plot[i][j])
                {
                    i += 1;
                    seen.insert((i, j, side_direction));
                }
            }
            _ => panic!("Invalid side direction"),
        },
        'u' => match side_direction {
            'l' => {
                while i > 0
                    && plot[i - 1][j] == plot[i][j]
                    && (j == 0 || plot[i - 1][j - 1] != plot[i][j])
                {
                    i -= 1;
                    seen.insert((i, j, side_direction));
                }
            }
            'r' => {
                while i > 0
                    && plot[i - 1][j] == plot[i][j]
                    && (j == w - 1 || plot[i - 1][j + 1] != plot[i][j])
                {
                    i -= 1;
                    seen.insert((i, j, side_direction));
                }
            }
            _ => panic!("Invalid side direction"),
        },
        'r' => match side_direction {
            'u' => {
                while j < w - 1
                    && plot[i][j + 1] == plot[i][j]
                    && (i == 0 || plot[i - 1][j + 1] != plot[i][j])
                {
                    j += 1;
                    seen.insert((i, j, side_direction));
                }
            }
            'd' => {
                while j < w - 1
                    && plot[i][j + 1] == plot[i][j]
                    && (i == h - 1 || plot[i + 1][j + 1] != plot[i][j])
                {
                    j += 1;
                    seen.insert((i, j, side_direction));
                }
            }
            _ => panic!("Invalid side direction"),
        },
        _ => panic!("Invalid traversal direction"),
    }

    seen
}

fn count_sides(plot: &Vec<Vec<char>>, contiguous_plot: &HashSet<(usize, usize)>) -> usize {
    let max_i = plot.len();
    let max_j = plot[0].len();

    let mut sides: usize = 0;

    let mut seen = HashSet::new();

    for (i, j) in contiguous_plot {
        if (*j == 0 || plot[*i][j - 1] != plot[*i][*j]) && !seen.contains(&(*i, *j, 'l')) {
            sides += 1;
            seen.extend(traverse_straight((*i, *j), 'u', 'l', plot, (max_i, max_j)));
            seen.extend(traverse_straight((*i, *j), 'd', 'l', plot, (max_i, max_j)));
        }
        if (*i == max_i - 1 || plot[i + 1][*j] != plot[*i][*j]) && !seen.contains(&(*i, *j, 'd')) {
            sides += 1;
            seen.extend(traverse_straight((*i, *j), 'r', 'd', plot, (max_i, max_j)));
            seen.extend(traverse_straight((*i, *j), 'l', 'd', plot, (max_i, max_j)));
        }
        if (*i == 0 || plot[i - 1][*j] != plot[*i][*j]) && !seen.contains(&(*i, *j, 'u')) {
            sides += 1;
            seen.extend(traverse_straight((*i, *j), 'r', 'u', plot, (max_i, max_j)));
            seen.extend(traverse_straight((*i, *j), 'l', 'u', plot, (max_i, max_j)));
        }
        if (*j == max_j - 1 || plot[*i][j + 1] != plot[*i][*j]) && !seen.contains(&(*i, *j, 'r')) {
            sides += 1;
            seen.extend(traverse_straight((*i, *j), 'u', 'r', plot, (max_i, max_j)));
            seen.extend(traverse_straight((*i, *j), 'd', 'r', plot, (max_i, max_j)));
        }
    }

    sides
}
fn p2(input: &str) -> usize {
    let plot = parse(input);

    let grid_dim = (plot.len(), plot[0].len());

    let mut visited_positions = HashSet::new();
    let mut contiguous_plots: Vec<HashSet<(usize, usize)>> = Vec::new();

    for i in 0..grid_dim.0 {
        for j in 0..grid_dim.1 {
            if visited_positions.contains(&(i, j)) {
                continue;
            }
            let contiguous = get_contiguous((i, j), &plot, grid_dim, HashSet::new());

            contiguous_plots.push(contiguous.clone());
            visited_positions.extend(contiguous);
        }
    }

    contiguous_plots
        .iter()
        .map(|c| count_sides(&plot, c) * c.len())
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

use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn group_plots(plot: &Vec<Vec<char>>) -> HashMap<(char, usize), Vec<(usize, usize)>> {
    let num_cells = plot.len() * plot[0].len();

    let mut groups: HashMap<(char, usize), Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in plot.iter().enumerate() {
        'mid: for (j, &cell) in row.iter().enumerate() {
            println!("Process {cell} at {i}, {j}");
            for nc in 0..num_cells {
                // let mut group = groups.entry((*cell, nc)).or_default();
                // group.push((i, j));
                if let Some(group) = groups.get_mut(&(cell, nc)) {
                    println!("Checking {:?} against {:?}", (i, j), (cell, nc));
                    //check if current cell is contiguous with any of the cells in the group
                    if group.iter().any(|(gi, gj)| {
                        (((*gi as i32) - (i as i32)).abs() + ((*gj as i32) - (j as i32)).abs()) == 1
                    }) {
                        println!("Pushing {:?} to {:?}", (i, j), group);
                        group.push((i, j));
                        continue 'mid;
                    }
                }
            }

            for nc in 0..num_cells {
                if !groups.contains_key(&(cell, nc)) {
                    println!("Pushing {:?} to new group with key {:?}", (i, j), (cell, nc));
                    groups.insert((cell, nc), vec![(i, j)]);
                    continue 'mid;
                }
            }

            // if let Some(group) = groups.get_mut(cell) {
            //
            //     group.iter().filter()
            //
            // }
            // groups.entry(*cell).or_default().push((i, j));
        }
    }
    groups
}

fn count_perimeter(plot: &Vec<Vec<char>>, sub_plot: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;

    let max_i = plot.len() - 1;
    let max_j = plot[0].len() - 1;

    for (i, j) in sub_plot {
        if *j == 0 || plot[*i][j - 1] != plot[*i][*j] {
            perimeter += 1;
            // println!("left");
        }
        if *i == max_i || plot[i + 1][*j] != plot[*i][*j] {
            perimeter += 1;
            // println!("down");
        }
        if *i == 0 || plot[i - 1][*j] != plot[*i][*j] {
            perimeter += 1;
            // println!("up");
        }
        if *j == max_j || plot[*i][j + 1] != plot[*i][*j] {
            perimeter += 1;
            // println!("right");
        }
    }

    println!(
        "{:?}: {:?} x {:?}={:?}",
        plot[sub_plot[0].0][sub_plot[0].1],
        sub_plot,
        perimeter,
        sub_plot.len() * perimeter
    );
    perimeter
}

fn p1(input: &str) -> usize {
    let plot = parse(input);
    let grouped_plot = group_plots(&plot);

    grouped_plot
        .values()
        .map(|sub_plot| count_perimeter(&plot, sub_plot) * sub_plot.len())
        .sum()
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

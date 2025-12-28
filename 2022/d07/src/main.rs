use std::{collections::HashMap, env, fs};

use Command::*;
use Content::*;
use TerminalLine::*;

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Command {
    ls,
    cd(String),
}

impl Command {
    fn from(line: &str) -> Command {
        let line = line.strip_prefix("$ ").unwrap();

        match &line[..2] {
            "cd" => {
                let dir = line[3..].trim().to_owned();
                cd(dir)
            }
            "ls" => ls,
            cmd => panic!("Invalid command {cmd}"),
        }
    }
}

#[derive(Debug)]
enum Content {
    Directory(String),
    File(usize),
}

impl Content {
    fn from(line: &str) -> Content {
        let (first, second) = line.split_once(' ').unwrap();

        if first == "dir" {
            Directory(second.trim().to_owned())
        } else {
            let size: usize = first.parse().unwrap();
            File(size)
        }
    }
}

#[derive(Debug)]
enum TerminalLine {
    Input(Command),
    Output(Content),
}

impl TerminalLine {
    fn from(line: &str) -> TerminalLine {
        if line.contains('$') {
            Input(Command::from(line))
        } else {
            Output(Content::from(line))
        }
    }
}

fn parse(input: &str) -> Vec<TerminalLine> {
    input.lines().map(TerminalLine::from).collect()
}

fn get_directory_sizes(lines: &[TerminalLine]) -> HashMap<String, usize> {
    // directory paths are the keys and values are the sizes
    let mut sizes: HashMap<String, usize> = HashMap::from([("/".to_owned(), 0)]);
    let mut paths: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Input(ls) => continue,
            Input(cd(dir)) => {
                if dir == ".." {
                    paths.pop();
                } else {
                    let next_cwd = if let Some(cwd) = paths.last() {
                        format!("{cwd}{dir}")
                    } else {
                        dir.to_owned()
                    };
                    paths.push(next_cwd);
                }
            }
            Output(Directory(name)) => {
                let parent = paths.last().unwrap();
                let full_path = format!("{parent}{name}");
                sizes.insert(full_path, 0);
            }
            Output(File(size)) => {
                for p in paths.iter() {
                    *sizes.get_mut(p).unwrap() += size;
                }
            }
        }
    }

    sizes
}

fn p1(input: &str) -> usize {
    let lines = parse(input);

    get_directory_sizes(&lines)
        .into_values()
        .filter(|&v| v <= 100000)
        .sum()
}

fn p2(input: &str) -> usize {
    let lines = parse(input);

    let directory_sizes = get_directory_sizes(&lines);

    let disk_space_total: usize = 70000000;
    let disk_space_required: usize = 30000000;

    let disk_space_unused = disk_space_total - directory_sizes.get("/").unwrap();
    let disk_space_needed = disk_space_required - disk_space_unused;

    *directory_sizes
        .iter()
        .filter_map(|(p, s)| {
            if p != "/" && s >= &disk_space_needed {
                Some(s)
            } else {
                None
            }
        })
        .min()
        .unwrap()
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
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 95437);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), 24933642);
    }
}

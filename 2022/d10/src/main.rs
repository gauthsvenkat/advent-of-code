use std::{collections::BTreeMap, env, fs};

use Instruction::*;

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Instruction {
    noop,
    addx(isize),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match &line[..4] {
            "noop" => noop,
            "addx" => addx(line[5..].parse().unwrap()),
            s => panic!("Unknown instruction {s}"),
        })
        .collect()
}

fn exec(instructions: &[Instruction]) -> BTreeMap<usize, isize> {
    let mut state: BTreeMap<usize, isize> = BTreeMap::from([(0, 1)]);

    let mut cycle: usize = 0;
    let mut x: isize = 1;

    for instruction in instructions {
        match instruction {
            noop => cycle += 1,
            addx(value) => {
                x += value;
                cycle += 2;
            }
        }

        state.insert(cycle, x);
    }

    state
}

fn p1(input: &str) -> usize {
    let instructions = parse(input);
    let state = exec(&instructions);

    [20, 60, 100, 140, 180, 220]
        .map(|c| {
            let value = state
                .iter()
                .filter(|(&cycle, _)| cycle < c)
                .max_by_key(|(&cycle, _)| cycle)
                .map(|(_, &value)| value)
                .unwrap();

            value * c as isize
        })
        .iter()
        .sum::<isize>() as usize
}

fn draw(state: &BTreeMap<usize, isize>) -> String {
    let max_cycles = state.keys().max().unwrap();
    let mut output = String::new();
    let mut sprite_horizontal_position: isize = 1;

    for i in 0..*max_cycles {
        let col = i % 40;

        if col == 0 {
            output.push('\n');
        }

        if (col as isize).abs_diff(sprite_horizontal_position) <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }

        if let Some(x) = state.get(&(i + 1)) {
            sprite_horizontal_position = *x;
        }
    }

    output.push('\n');
    output
}

fn p2(input: &str) -> String {
    let instructions = parse(input);
    let state = exec(&instructions);
    draw(&state)
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
        assert_eq!(p1(EXAMPLE), 13140);
    }

    #[test]
    fn test_p2_example() {
        let expected = "\n\
##..##..##..##..##..##..##..##..##..##..\n\
###...###...###...###...###...###...###.\n\
####....####....####....####....####....\n\
#####.....#####.....#####.....#####.....\n\
######......######......######......####\n\
#######.......#######.......#######.....\n\
";
        assert_eq!(p2(EXAMPLE), expected);
    }
}

use std::{env, fs};

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn p1(input: &str) -> usize {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    // let mut buffer = program.clone();

    // while let Some(opcode) = program.pop_front() {
    //     match opcode {
    //         opcode if opcode == 1 || opcode == 2 => {
    //             let operand1 = program.pop_front().unwrap();
    //             let operand2 = program.pop_front().unwrap();
    //             let storage = program.pop_front().unwrap();
    //
    //             dbg!(operand1, operand2, storage, &program);
    //
    //             let a = program[operand1];
    //             let b = program[operand2];
    //
    //             let res = if opcode == 1 { a + b } else { a * b };
    //
    //             program[storage] = res;
    //         }
    //         99 => break,
    //         n => panic!("Invalid opcode {n}"),
    //     }
    // }

    let mut ip = 0;

    loop {
        let opcode = program[ip];
        match opcode {
            opcode if opcode == 1 || opcode == 2 => {
                ip += 1;
                let operand1 = program[ip];

                ip += 1;
                let operand2 = program[ip];

                ip += 1;
                let storage = program[ip];

                dbg!(operand1, operand2, storage, &program);
                let a = program[operand1];
                let b = program[operand2];

                let res = if opcode == 1 { a + b } else { a * b };

                program[storage] = res;

                ip += 1;
            }
            99 => break,
            n => panic!("Invalid opcode {n}"),
        }
    }

    program[0]
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../eg1.txt");

    #[test]
    fn test_p1() {
        // TODO:
        assert_eq!(p1(EXAMPLE), 0);
    }

    #[test]
    fn test_p2() {
        // TODO:
        assert_eq!(p2(EXAMPLE), 0);
    }
}

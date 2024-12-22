use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,

    ip: usize,
}

#[derive(Debug)]
enum Operand {
    Literal(u8),
    Combo(u8),
    Ignore,
}

impl Operand {
    fn value(&self, machine: &Machine) -> usize {
        match self {
            Operand::Literal(v) | Operand::Combo(v @ 0..=3) => *v as usize,
            Operand::Combo(v @ 4..=6) => match v {
                4 => machine.a,
                5 => machine.b,
                6 => machine.c,
                _ => unreachable!(),
            },
            _ => panic!("Invalid operand"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

struct Program {
    raw: Vec<u8>,
    instructions: Vec<Instruction>,
    length: usize,
}

fn parse(input: &str) -> (Machine, Program) {
    let re = Regex::new(r"\b\d+\b").unwrap();

    let groups: Vec<_> = input.split("\n\n").collect();

    let registers: Vec<usize> = re
        .find_iter(groups[0])
        .map(|v| v.as_str().parse().unwrap())
        .collect();

    let machine = Machine {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        ip: 0,
    };

    let program_raw: Vec<u8> = re
        .find_iter(groups[1])
        .map(|v| v.as_str().parse().unwrap())
        .collect();

    let instructions: Vec<Instruction> = program_raw
        .chunks(2)
        .map(|w| {
            let (opc, ope) = (w[0], w[1]);

            let (opcode, operand) = match opc {
                0 => (Opcode::adv, Operand::Combo(ope)),
                1 => (Opcode::bxl, Operand::Literal(ope)),
                2 => (Opcode::bst, Operand::Combo(ope)),
                3 => (Opcode::jnz, Operand::Literal(ope)),
                4 => (Opcode::bxc, Operand::Ignore),
                5 => (Opcode::out, Operand::Combo(ope)),
                6 => (Opcode::bdv, Operand::Combo(ope)),
                7 => (Opcode::cdv, Operand::Combo(ope)),
                _ => panic!("Invalid opcode"),
            };

            Instruction { opcode, operand }
        })
        .collect();

    let program = Program {
        raw: program_raw,
        length: instructions.len(),
        instructions,
    };

    (machine, program)
}

fn run_once(machine: &mut Machine, program: &Program) -> Option<u8> {
    let Program {
        instructions,
        raw: _,
        length,
    } = program;

    loop {
        if machine.ip >= *length {
            return None;
        }

        let Instruction { opcode, operand } = &instructions[machine.ip];

        match opcode {
            // 0
            Opcode::adv => {
                machine.a >>= operand.value(machine);
            }
            // 1
            Opcode::bxl => {
                machine.b ^= operand.value(machine);
            }
            // 2
            Opcode::bst => {
                machine.b = operand.value(machine) % 8;
            }
            // 3
            Opcode::jnz => {
                if machine.a != 0 {
                    machine.ip = operand.value(machine);
                    continue;
                }
            }
            // 4
            Opcode::bxc => {
                machine.b ^= machine.c;
            }
            // 5
            Opcode::out => {
                machine.ip += 1;
                return Some((operand.value(machine) % 8) as u8);
            }
            // 6
            Opcode::bdv => {
                machine.b = machine.a >> operand.value(machine);
            }
            // 7
            Opcode::cdv => {
                machine.c = machine.a >> operand.value(machine);
            }
        }

        machine.ip += 1;
    }
}

fn run(machine: &mut Machine, program: &Program) -> Vec<u8> {
    let mut output = Vec::new();
    while let Some(n) = run_once(machine, program) {
        output.push(n);
    }
    output
}

fn p1(input: &str) -> String {
    let (mut machine, program) = parse(input);

    run(&mut machine, &program)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn is_quine(mut machine: Machine, program: &Program) -> bool {
    let mut i = 0;

    while let Some(n) = run_once(&mut machine, program) {
        if i >= program.length * 2 || n != program.raw[i] {
            return false;
        }

        i += 1;
    }

    if i != program.length * 2 {
        return false;
    }

    true
}

fn binary_search(low: usize, high: usize, program: &Program, lb: bool) -> usize {
    let mid = (low + high) / 2;

    let output = run(
        &mut Machine {
            a: mid,
            b: 0,
            c: 0,
            ip: 0,
        },
        program,
    );

    match lb {
        true => {
            if low == mid {
                mid + 1
            } else if output.len() < program.length * 2 {
                binary_search(mid, high, program, lb)
            } else {
                binary_search(low, mid, program, lb)
            }
        }
        false => {
            if low == mid {
                mid
            } else if output.len() <= program.length * 2 {
                binary_search(mid, high, program, lb)
            } else {
                binary_search(low, mid, program, lb)
            }
        }
    }
}

fn p2(input: &str) -> usize {
    let (machine, program) = parse(input);

    // for a in 0..=usize::MAX {
    //     let mut machine = Machine { a, ..machine };
    //
    //     print!("a = {}: ", a);
    //     let output = run(&mut machine, &program);
    //     println!("{:?}", output);
    // }
    println!("program length = {}", program.length * 2);

    let a_lb = binary_search(usize::MIN, usize::MAX, &program, true);
    let a_ub = binary_search(usize::MIN, usize::MAX, &program, false);

    println!("lower bound = {}", a_lb);
    let output = run(&mut Machine { a:a_lb, ..machine }, &program);
    println!("output = {:?}", output);
    println!("output length {}", output.len());

    println!("upper bound = {}", a_ub);
    let output = run(&mut Machine { a:a_ub, ..machine }, &program);
    println!("output = {:?}", output);
    println!("output length {}", output.len());

    println!("Difference = {}", a_ub - a_lb);
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

use rayon::prelude::*;
use std::{env, fs};
use z3::{ast::Int, Optimize, SatResult::Sat};

type IntVec = Vec<u64>;

#[derive(Debug)]
struct MachineSpec {
    indicator_lights: IntVec,
    buttons: Vec<IntVec>,
    joltages: Vec<u64>,
}

fn parse(input: &str) -> Vec<MachineSpec> {
    input
        .lines()
        .map(|line| {
            let start = line.find('[').unwrap() + 1;
            let end = line.find(']').unwrap();
            let indicator_lights: Vec<_> = line[start..end]
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect();

            let start = line.find(' ').unwrap() + 1;
            let end = line.rfind(' ').unwrap();
            let buttons = line[start..end]
                .split_whitespace()
                .map(|s| {
                    let idxs: Vec<usize> = s
                        .replace(['(', ')'], "")
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect();

                    (0..(indicator_lights.len()))
                        .map(|i| if idxs.contains(&i) { 1 } else { 0 })
                        .collect()
                })
                .collect();

            let start = line.find('{').unwrap() + 1;
            let end = line.find('}').unwrap();
            let joltages = line[start..end]
                .split(',')
                .map(|c| c.parse::<u64>().unwrap())
                .collect();

            MachineSpec {
                indicator_lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn optimize(target: &[u64], buttons: &[IntVec], joltage_switch: bool) -> usize {
    // Buttons represented as a binary vector of 0s and 1s.
    let buttons: Vec<Vec<Int>> = buttons
        .iter()
        .map(|b| b.iter().map(|x| Int::from_u64(*x)).collect())
        .collect();

    let m = buttons.len();
    let n = target.len();

    // Target is the final output state we want the machine to be in.
    // In case of indicator lights, this will be a binary vector of 0s and 1s.
    // In case of joltages, this will be a vector of ints.
    let target: Vec<Int> = target.iter().map(|x| Int::from_u64(*x)).collect();

    // X is the vector representing the number of presses, corresponding to each
    // button. This is what we'll be solving for.
    #[allow(non_snake_case)]
    let X: Vec<Int> = (0..m).map(|_| Int::fresh_const("x")).collect();

    // Create the optimizer
    let optimizer = Optimize::new();

    // Constraint #1: number of presses >= 0
    for x in &X {
        optimizer.assert(&x.ge(0));
    }

    // Constraint #2: number of presses * buttons = target
    // X is (1,m)
    // buttons is (m,n)
    // target is (1,n)
    for j in 0..n {
        let terms: Vec<_> = (0..m).map(|i| &X[i] * &buttons[i][j]).collect();
        let sum = Int::add(&terms.iter().collect::<Vec<_>>());

        // for p1, a button would only toggle the light. So, if the button was
        // pressed an even number of times, it will be the same as the light's
        // starting state. If pressed an odd number of times, the light will be
        // toggled from its initial state.
        // for p2, every button press counts towards increasing the joltage
        let t = if joltage_switch { sum } else { sum.rem(2) };

        optimizer.assert(&t.eq(&target[j]));
    }

    let total_presses = Int::add(&X.iter().collect::<Vec<_>>());

    optimizer.minimize(&total_presses);

    match optimizer.check(&[]) {
        Sat => {
            let model = optimizer.get_model().unwrap();

            let total_presses: u64 = X
                .iter()
                .map(|x| model.eval(x, true).unwrap().as_u64().unwrap())
                .sum();

            total_presses as usize
        }
        _ => unreachable!("All problems should be solvable"),
    }
}

fn p1(input: &str) -> usize {
    let machine_specs = parse(input);

    machine_specs
        .par_iter()
        .map(|m| optimize(&m.indicator_lights, &m.buttons, false))
        .sum()
}

fn p2(input: &str) -> usize {
    let machine_specs = parse(input);

    machine_specs
        .par_iter()
        .map(|m| optimize(&m.joltages, &m.buttons, true))
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
        assert_eq!(p1(EXAMPLE), 7);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 33);
    }
}

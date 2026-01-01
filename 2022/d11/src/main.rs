use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug)]
enum Operand {
    Num(usize),
    Old,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug)]
struct Test {
    divisible_by: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,

    n_inpections: usize,
}

type Monkeys = HashMap<usize, Monkey>;

fn parse(input: &str) -> Monkeys {
    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.lines().map(|l| l.trim()).collect();

            let monkey_id = lines[0];
            let items = lines[1];
            let operation = lines[2];
            let test = lines[3];
            let on_true = lines[4];
            let on_false = lines[5];

            // 1. monkey_id
            let (_, monkey_id) = monkey_id
                .strip_suffix(':')
                .unwrap()
                .split_once(' ')
                .unwrap();
            let monkey_id: usize = monkey_id.parse().unwrap();

            // 2. items
            let items = items
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect();

            // 3. operation
            let operation = operation.strip_prefix("Operation: new = old ").unwrap();
            let (operator, operand) = operation.split_once(' ').unwrap();
            let operator = if operator == "*" {
                Operator::Mul
            } else {
                Operator::Add
            };
            let operand = if operand == "old" {
                Operand::Old
            } else {
                Operand::Num(operand.parse().unwrap())
            };
            let operation = Operation { operator, operand };

            // 4. test
            let divisible_by = test
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            // 5. on true
            let on_true = on_true
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            // 6. on false
            let on_false = on_false
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let test = Test {
                divisible_by,
                on_true,
                on_false,
            };

            (
                monkey_id,
                Monkey {
                    items,
                    operation,
                    test,
                    n_inpections: 0,
                },
            )
        })
        .collect()
}

impl Monkey {
    fn throw(&mut self, divide_by: usize, modulo: usize) -> Option<(usize, usize)> {
        let Monkey {
            operation,
            test,
            items,
            n_inpections,
        } = self;
        let Operation { operator, operand } = operation;
        let Test {
            divisible_by,
            on_true,
            on_false,
        } = test;

        if let Some(mut worry_level) = items.pop_front() {
            *n_inpections += 1;

            worry_level = match (operator, operand) {
                (Operator::Add, Operand::Num(n)) => worry_level + *n,
                (Operator::Mul, Operand::Num(n)) => worry_level * *n,
                (Operator::Add, Operand::Old) => worry_level + worry_level,
                (Operator::Mul, Operand::Old) => worry_level * worry_level,
            };
            worry_level /= divide_by;

            worry_level %= modulo;

            if worry_level.is_multiple_of(*divisible_by) {
                Some((worry_level, *on_true))
            } else {
                Some((worry_level, *on_false))
            }
        } else {
            None
        }
    }

    fn catch(&mut self, worry_level: usize) {
        self.items.push_back(worry_level);
    }

    fn throw_all(&mut self, divide_by: usize, modulo: usize) -> Vec<(usize, usize)> {
        let mut to_throw = Vec::new();

        while let Some(thing) = self.throw(divide_by, modulo) {
            to_throw.push(thing);
        }

        to_throw
    }
}

fn simulate_round(mut monkeys: Monkeys, divide_by: usize, modulo: usize) -> Monkeys {
    let len = monkeys.len();

    for id in 0..len {
        let things = monkeys.get_mut(&id).unwrap().throw_all(divide_by, modulo);

        for (worry_level, to) in things {
            monkeys.get_mut(&to).unwrap().catch(worry_level);
        }
    }

    monkeys
}

fn monkey_business(rounds: usize, mut monkeys: Monkeys, divide_by: usize) -> usize {
    let modulo = monkeys.values().map(|m| m.test.divisible_by).product();

    for _ in 0..rounds {
        monkeys = simulate_round(monkeys, divide_by, modulo);
    }

    let inspections_count = {
        let mut inspections_count: Vec<usize> = monkeys.values().map(|m| m.n_inpections).collect();
        inspections_count.sort_unstable();
        inspections_count
    };

    inspections_count
        .iter()
        .skip(inspections_count.len() - 2)
        .product()
}

fn p1(input: &str) -> usize {
    monkey_business(20, parse(input), 3)
}

fn p2(input: &str) -> usize {
    monkey_business(10000, parse(input), 1)
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
        assert_eq!(p1(EXAMPLE), 10605);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 2713310158);
    }
}

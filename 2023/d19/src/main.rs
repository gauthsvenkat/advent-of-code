use std::{
    collections::{HashMap, HashSet},
    env, fs,
    ops::RangeInclusive,
};

#[derive(Debug)]
enum Rule<'a> {
    Condition {
        category: char,
        sign: char,
        value: usize,
        next: &'a str,
    },
    Catch {
        next: &'a str,
    },
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get(&self, category: &char) -> usize {
        match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            c => panic!("Invalid category {c}!"),
        }
    }
}

type WorkflowNames = HashSet<String>;
type Workflows<'a> = HashMap<&'a str, Vec<Rule<'a>>>;

fn parse_workflows_block(input: &'_ str) -> (WorkflowNames, Workflows<'_>) {
    // First pass, collect all the workflow names
    let workflow_names: HashSet<String> = input
        .lines()
        .map(|l| {
            let (name, _) = l.split_once('{').unwrap();
            name.to_string()
        })
        .collect();

    // second pass, parse workflows
    let workflows: Workflows = input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let workflow = rest.strip_suffix('}').unwrap();

            let rules: Vec<Rule> = workflow
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let (expr, next) = rule.split_once(':').unwrap();

                        let category = expr.chars().next().unwrap();
                        let sign = expr.chars().nth(1).unwrap();
                        let value: usize = expr[2..].parse().unwrap();

                        Rule::Condition {
                            category,
                            sign,
                            value,
                            next,
                        }
                    } else {
                        Rule::Catch { next: rule }
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    (workflow_names, workflows)
}

fn parse_parts_block(input: &str) -> Vec<Part> {
    input
        .lines()
        .map(|line| {
            let line = line
                .strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
                .unwrap();

            let parts: Vec<&str> = line.split(',').collect();

            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;

            for part in parts {
                let value: usize = part[2..].parse().unwrap();

                match part.chars().next().unwrap() {
                    'x' => x = value,
                    'm' => m = value,
                    'a' => a = value,
                    's' => s = value,
                    c => panic!("Couldn't parse category {c}"),
                }
            }

            Part { x, m, a, s }
        })
        .collect()
}

fn parse(input: &'_ str) -> (WorkflowNames, Workflows<'_>, Vec<Part>) {
    let (workflows_block, parts_block) = input.split_once("\n\n").unwrap();

    let (names, workflows) = parse_workflows_block(workflows_block);
    let parts = parse_parts_block(parts_block);

    (names, workflows, parts)
}

fn process_part(part: &Part, name: &str, workflows: &Workflows) -> bool {
    match name {
        "A" => return true,
        "R" => return false,
        name => {
            for rule in workflows.get(name).unwrap() {
                match rule {
                    Rule::Condition {
                        category,
                        sign,
                        value,
                        next,
                    } => {
                        let condition_result = match sign {
                            '>' => part.get(category) > *value,
                            '<' => part.get(category) < *value,
                            s => panic!("Invalid sign {s}!"),
                        };

                        if condition_result {
                            return process_part(part, next, workflows);
                        }
                    }
                    Rule::Catch { next } => return process_part(part, next, workflows),
                }
            }
        }
    }

    unreachable!("Should've have determined part acceptance by now!")
}

fn p1(input: &str) -> usize {
    let (_, workflows, parts) = parse(input);

    parts
        .iter()
        .filter_map(|p| {
            if process_part(p, "in", &workflows) {
                let &Part { x, m, a, s } = p;
                Some(x + m + a + s)
            } else {
                None
            }
        })
        .sum()
}

type CategoryRange = RangeInclusive<usize>;

#[derive(Debug, Clone)]
struct PartRange {
    x: CategoryRange,
    m: CategoryRange,
    a: CategoryRange,
    s: CategoryRange,
}

impl PartRange {
    fn get(&self, category: &char) -> CategoryRange {
        match category {
            'x' => *self.x.start()..=*self.x.end(),
            'm' => *self.m.start()..=*self.m.end(),
            'a' => *self.a.start()..=*self.a.end(),
            's' => *self.s.start()..=*self.s.end(),
            c => panic!("Invalid category {c}!"),
        }
    }

    fn put(&mut self, category: &char, range: CategoryRange) {
        match category {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            c => panic!("Invalid category {c}!"),
        }
    }
}

enum RangeSplit {
    Valid(PartRange),
    Invalid(PartRange),
    Partial(PartRange, PartRange),
}

fn split_range(part_range: &PartRange, category: &char, sign: &char, value: &usize) -> RangeSplit {
    let mut part_range = part_range.clone();

    let range = part_range.get(category);

    match sign {
        '>' => {
            if value < range.start() {
                // range is entirely valid
                part_range.put(category, *range.start()..=*range.end());
                RangeSplit::Valid(part_range)
            } else if *range.end() <= *value {
                part_range.put(category, *range.start()..=*range.end());
                RangeSplit::Invalid(part_range)
            } else {
                // range is partially valid
                let mut part_range2 = part_range.clone();

                part_range.put(category, value + 1..=*range.end());
                part_range2.put(category, *range.start()..=*value);

                RangeSplit::Partial(part_range, part_range2)
            }
        }
        '<' => {
            if *range.end() < *value {
                // range is entirely valid
                part_range.put(category, *range.start()..=*range.end());
                RangeSplit::Valid(part_range)
            } else if value <= range.start() {
                // range is entirely invalid

                part_range.put(category, *range.start()..=*range.end());
                RangeSplit::Invalid(part_range)
            } else {
                // range is partially valid

                // range is partially valid
                let mut part_range2 = part_range.clone();

                part_range.put(category, *range.start()..=value - 1);
                part_range2.put(category, *value..=*range.end());

                RangeSplit::Partial(part_range, part_range2)
            }
        }
        s => panic!("Invalid sign {s}!"),
    }
}

fn process_part_range(part_range: PartRange, name: &str, workflows: &Workflows) -> usize {
    match name {
        "A" => {
            let PartRange { x, m, a, s } = part_range;

            x.count() * m.count() * a.count() * s.count()
        }
        "R" => 0,
        name => {
            let mut part_range: Option<PartRange> = Some(part_range);
            let mut acc = 0;

            for rule in workflows.get(name).unwrap() {
                if part_range.is_none() {
                    continue;
                }

                match rule {
                    Rule::Condition {
                        category,
                        sign,
                        value,
                        next,
                    } => match split_range(&part_range.unwrap(), category, sign, value) {
                        RangeSplit::Valid(range) => {
                            acc += process_part_range(range, next, workflows);
                            part_range = None;
                        }
                        RangeSplit::Invalid(range) => part_range = Some(range),
                        RangeSplit::Partial(valid, invalid) => {
                            acc += process_part_range(valid, next, workflows);
                            part_range = Some(invalid);
                        }
                    },
                    Rule::Catch { next } => {
                        acc += process_part_range(part_range.clone().unwrap(), next, workflows);
                    }
                }
            }

            acc
        }
    }
}

fn p2(input: &str) -> usize {
    let (_, workflows, _) = parse(input);

    process_part_range(
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
        "in",
        &workflows,
    )
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
        assert_eq!(p1(EXAMPLE), 19114);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE), 167409079868000);
    }
}

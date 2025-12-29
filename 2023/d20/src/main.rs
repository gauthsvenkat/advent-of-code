use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

use Module::*;

#[derive(Debug)]
struct Message {
    from: String,
    to: String,
    pulse: bool,
}

#[derive(Debug)]
enum Module {
    Broadcast {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        state: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        memory: HashMap<String, bool>,
        outputs: Vec<String>,
    },
}

impl Module {
    fn from(
        prefix: &char,
        name: &str,
        outputs: Vec<String>,
        module_map: &HashMap<(char, &str), Vec<&str>>,
    ) -> Module {
        match prefix {
            'b' => Broadcast {
                name: name.to_string(),
                outputs,
            },
            '%' => FlipFlop {
                name: name.to_string(),
                state: false,
                outputs,
            },
            '&' => {
                // All the modules that output to the given conjuction module
                let conjuction_inputs: Vec<&&str> = module_map
                    .iter()
                    .filter_map(|((_, m_name), m_outputs)| {
                        if m_outputs.contains(&name) {
                            Some(m_name)
                        } else {
                            None
                        }
                    })
                    .collect();

                Conjunction {
                    name: name.to_string(),
                    memory: conjuction_inputs
                        .into_iter()
                        .map(|s| (s.to_string(), false))
                        .collect(),
                    outputs,
                }
            }
            c => panic!("Invalid prefix {c}"),
        }
    }
}

type Graph = HashMap<String, Module>;

fn parse(input: &str) -> Graph {
    let module_map: HashMap<(char, &str), Vec<&str>> = input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once("-> ").unwrap();
            let prefix = name.chars().next().unwrap();
            let name = name.trim_matches(['%', '&', ' ']);
            let outputs: Vec<&str> = outputs.split(',').map(|n| n.trim()).collect();

            ((prefix, name), outputs)
        })
        .collect();

    module_map
        .iter()
        .map(|((prefix, name), outputs)| {
            (
                name.to_string(),
                Module::from(
                    prefix,
                    name,
                    outputs.iter().map(|n| n.to_string()).collect(),
                    &module_map,
                ),
            )
        })
        .collect()
}

impl Module {
    fn process(&mut self, message: Message) -> VecDeque<Message> {
        match self {
            Broadcast { name, outputs } => outputs
                .iter()
                .map(|module_name| Message {
                    from: name.to_string(),
                    to: module_name.to_string(),
                    pulse: message.pulse,
                })
                .collect(),
            FlipFlop {
                name,
                state,
                outputs,
            } => {
                if message.pulse {
                    // do nothing if pulse is high
                    VecDeque::new()
                } else {
                    // flip self state if pulse is low
                    // and propagate
                    *state = !*state;

                    outputs
                        .iter()
                        .map(|module_name| Message {
                            from: name.to_string(),
                            to: module_name.to_string(),
                            pulse: *state,
                        })
                        .collect()
                }
            }

            Conjunction {
                name,
                memory,
                outputs,
            } => {
                // update the state for that specific input
                *memory.get_mut(message.from.as_str()).unwrap() = message.pulse;

                // if all high, send low
                // else send high
                let pulse = !memory.values().all(|v| *v);

                outputs
                    .iter()
                    .map(|module_name| Message {
                        from: name.to_string(),
                        to: module_name.to_string(),
                        pulse,
                    })
                    .collect()
            }
        }
    }
}

fn simulate(
    mut graph: Graph,
    n: usize,
    interesting_modules: HashSet<String>,
) -> (usize, HashMap<String, usize>) {
    // interesting_modules are modules for which we want to find out the lowest number
    // of button presses until they send a high signal.
    let (mut acc_low, mut acc_high): (usize, usize) = (0, 0);

    let mut im_cycle_map: HashMap<String, usize> = HashMap::new();

    for i in 1..=n {
        let mut message_queue = VecDeque::from([Message {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: false,
        }]);

        let (mut n_low, mut n_high): (usize, usize) = (0, 0);

        while let Some(message) = message_queue.pop_front() {
            if let Some(interesting_module) = interesting_modules.get(&message.from[..]) {
                if message.pulse {
                    im_cycle_map
                        .entry(interesting_module.to_string())
                        .or_insert(i);
                }
            }

            if message.pulse {
                n_high += 1;
            } else {
                n_low += 1;
            }

            // Not all recipients might be in the graph.
            // See second example (one with "output") from puzzle page.
            if let Some(module) = graph.get_mut(&message.to) {
                message_queue.extend(module.process(message));
            }
        }

        acc_low += n_low;
        acc_high += n_high;
    }

    (acc_low * acc_high, im_cycle_map)
}

fn p1(input: &str) -> usize {
    let (ans, _) = simulate(parse(input), 1000, HashSet::new());
    ans
}

fn p2(input: &str) -> usize {
    // NOTE: Maybe not a super general solution but the idea is as follows
    // Only a single (conjunction) module is connected to the input of "rx",
    // which we will call `rx_input`.
    // We consider the inputs to `rx_input`, as `interesting_modules`; Because
    // when all of these modules send high pulses at the same time, `rx_input`
    // will send a low pulse to "rx".
    // From examining the simulation, those modules send a high pulse once
    // every couple thousand button presses, in a cycle.
    // In our simulation, we try to find the first time, i.e., at which button press,
    // each of those modules send a high pulse.
    // We then take the LCM of those numbers, which is the answer.
    let graph = parse(input);

    let rx_input = graph
        .iter()
        .find_map(|(name, module)| match module {
            // In my input atleast, a Conjunction module with a single output
            // of "rx" was present. I assume that is the case for all inputs.
            Conjunction { outputs, .. } => {
                if outputs.contains(&"rx".to_string()) {
                    Some(name)
                } else {
                    None
                }
            }
            _ => None,
        })
        .unwrap();

    let interesting_modules: HashSet<String> = match graph.get(rx_input).unwrap() {
        Conjunction { memory, .. } => memory.keys().cloned().collect(),
        _ => unreachable!("{rx_input} is a conjuction module!"),
    };

    let (_, im_cycles) = simulate(graph, 10000, interesting_modules);

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    im_cycles.into_values().reduce(lcm).unwrap()
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

    const EXAMPLE1: &str = include_str!("../eg1.txt");
    const EXAMPLE2: &str = include_str!("../eg2.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(EXAMPLE1), 32000000);
        assert_eq!(p1(EXAMPLE2), 11687500);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), 1);
    }
}

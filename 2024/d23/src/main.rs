use std::{
    collections::{BTreeSet, HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut network: HashMap<String, HashSet<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let connection: Vec<&str> = line.split('-').collect();

        let (a, b) = (connection[0].to_string(), connection[1].to_string());

        network.entry(a.clone()).or_default().insert(b.clone());
        network.entry(b).or_default().insert(a);
    });

    network
}

fn get_triples(network: &HashMap<String, HashSet<String>>) -> HashSet<BTreeSet<String>> {
    let mut triples = HashSet::new();

    for (computer, connections) in network.iter() {
        for (i, connection) in connections.iter().enumerate() {
            for another_connection in connections.iter().skip(i) {
                if network
                    .get(connection)
                    .unwrap()
                    .contains(another_connection)
                {
                    let mut triple = BTreeSet::new();
                    triple.insert(computer.clone());
                    triple.insert((*connection).clone());
                    triple.insert((*another_connection).clone());

                    triples.insert(triple);
                }
            }
        }
    }

    triples
}

fn p1(input: &str) -> usize {
    let network = parse(input);
    let triples = get_triples(&network);

    triples
        .iter()
        .filter(|triple| triple.iter().any(|computer| computer.starts_with('t')))
        .count()
}

fn get_largest_set(network: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut largest_set = HashSet::new();

    for (computer, connections) in network.iter() {
        let mut set = HashSet::new();
        set.insert(computer.clone());

        let mut candidates = connections.iter().collect::<Vec<&String>>();

        while let Some(candidate) = candidates.pop() {
            if set
                .iter()
                .all(|c| network.get(c).unwrap().contains(candidate))
            {
                set.insert(candidate.clone());
            }
        }

        if set.len() > largest_set.len() {
            largest_set = set;
        }
    }

    largest_set
}

fn p2(input: &str) -> String {
    let network = parse(input);
    let largest_set = get_largest_set(&network);

    {
        let mut largest_set = largest_set.into_iter().collect::<Vec<String>>();
        largest_set.sort_unstable();
        largest_set
    }
    .join(",")
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
        assert_eq!(p1(EXAMPLE), 7);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(p2(EXAMPLE), "co,de,ka,ta");
    }
}

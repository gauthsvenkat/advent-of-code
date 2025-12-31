use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn p1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|s| {
            // Condition 1: Contains at least 3 vowels
            let cond1 = s.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;
            // Condition 2: Contains at least one letter that appears twice  in a row
            let chars: Vec<char> = s.chars().collect();
            let cond2 = chars.windows(2).any(|w| w[0] == w[1]);
            // Condition 3: Does not contain certain strings
            let cond3 =
                !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"));

            cond1 && cond2 && cond3
        })
        .count()
}

fn p2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|s| {
            // Condition 1: Contains a pair of any two letters at least twice
            let chars: Vec<char> = s.chars().collect();

            let counter = {
                let mut counter: HashMap<(char, char), usize> = HashMap::new();

                for w in chars.windows(2) {
                    let sub_s: String = w.iter().collect();
                    *counter.entry((w[0], w[1])).or_insert(0) = s.matches(&sub_s).count();
                }
                counter
            };

            let cond1 = counter.values().any(|&v| v >= 2);

            //Condition 2: Contains at least one letter which repeats with exactly one letter
            //between
            let cond2 = chars.windows(3).any(|w| (w[0] == w[2]) && (w[0] != w[1]));

            cond1 && cond2
        })
        .count()
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
        assert_eq!(p1(EXAMPLE1), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(EXAMPLE2), 3);
    }
}

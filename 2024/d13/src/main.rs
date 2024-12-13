use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32), (i32, i32))> {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let tuples: Vec<(i32, i32)> = re
                .captures_iter(block)
                .map(|cap| {
                    let x = cap[1].parse::<i32>().unwrap();
                    let y = cap[2].parse::<i32>().unwrap();
                    (x, y)
                })
                .collect();

            (tuples[0], tuples[1], tuples[2])
        })
        .collect()
}

fn egcd(p: i32, q: i32) -> (i32, i32, i32) {
    if p == 0 {
        return (q, 0, 1);
    }

    let (gcd, x1, y1) = egcd(q % p, p);
    let ca = y1 - (q / p) * x1;
    let cb = x1;

    (gcd, ca, cb)
}

fn find_presses(x: i32, y: i32, t: i32) -> HashSet<(i32, i32)> {
    let (gcd, ca, cb) = egcd(x, y);

    dbg!(x, y, t, gcd, ca, cb);

    if t % gcd != 0 {
        return HashSet::new();
    }

    let a0 = ca * (t / gcd);
    let b0 = cb * (t / gcd);

    /*
     * min 3a + b where
     * a = a0 + (y / gcd) * t
     * b = b0 - (x / gcd) * t
     * a, b > 0
     * t is any integer
     */

    /*
     * since a > 0 => a0 + (y / gcd) * t > 0
     * -a0 * gcd / y < t
     */

    /*
     * since b > 0 => b0 - (x / gcd) * t > 0
     * t < b0 * gcd / x
     */

    /*
     * therefore (-a0 * gcd / y) < t < (b0 * gcd / x)
     */

    let t_min: i32 = -a0 * gcd / y;
    let t_max: i32 = b0 * gcd / x;

    dbg!(a0, b0, t_min, t_max);
    (t_min..t_max)
        .map(|t| (a0 + (y / gcd) * t, b0 - (x / gcd) * t))
        .filter(|(a, b)| a > &0 && b > &0)
        .collect()
}

fn p1(input: &str) -> i32 {
    let machines = parse(input);

    machines.iter().map(|(x, y, t)| {
        find_presses(x.0, y.0, t.0)
            .intersection(&find_presses(x.1, y.1, t.1))
            .map(|(a, b)| a * 3 + b)
            .min()
            .unwrap_or(0)
    }).sum()
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

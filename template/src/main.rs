use std::env;
use std::fs;

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path).unwrap()
}

fn parse(input: &str) -> () {
    todo!()
}

fn p1(input: &str) -> usize {
    parsed_input = parse(input);
    // TODO:
    todo!()
}

fn p2(input: &str) -> usize {
    parsed_input = parse(input);
    // TODO:
    todo!()
}

fn main() {
    let input = get_input();

    let p1_sol = p1(&input);
    println!("{p1_sol}");

    let p2_sol = p2(&input);
    println!("{p2_sol}");
}

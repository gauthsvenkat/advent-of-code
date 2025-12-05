use std::env;
use std::fs;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let groups: Vec<_> = input.split("\n\n").collect();

    let map: Vec<Vec<char>> = groups[0].lines().map(|l| l.chars().collect()).collect();
    let moves: Vec<char> = groups[1].chars().filter(|c| !c.is_whitespace()).collect();

    (map, moves)
}

fn _render_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn step(map: &mut Vec<Vec<char>>, pos: (usize, usize), dir: char) -> (usize, usize) {
    let (dx, dy) = match dir {
        '<' => (0, -1),
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        _ => panic!("Invalid direction"),
    };

    let n_pos = (
        (pos.0 as isize + dx) as usize,
        (pos.1 as isize + dy) as usize,
    );

    let c_object = map[pos.0][pos.1];
    let n_object = map[n_pos.0][n_pos.1];

    // Do nothing and return the current position
    // if the next position is a wall
    if n_object == '#' {
        pos

    // If the next position is an empty space, move the object
    // and return the new position
    } else if n_object == '.' {
        map[n_pos.0][n_pos.1] = c_object;
        map[pos.0][pos.1] = '.';

        n_pos

    // If the next position is a
    // - box
    // - or a big box and we're moving it from the left or the right
    // try moving the (big) box and return
    // - the current position if it can't be moved
    // - the next position if it can be moved
    } else if (n_object == 'O')
        || ((n_object == '[' || n_object == ']') && (dir == '<' || dir == '>'))
    {
        if n_pos == step(map, n_pos, dir) {
            pos
        } else {
            map[n_pos.0][n_pos.1] = c_object;
            map[pos.0][pos.1] = '.';

            n_pos
        }
    // If the next position is a big box and we're moving it from the
    // top or the bottom, try moving both halves and return
    // - the current position if both halves can't be moved
    // - the next position if both halves can be moved
    } else if (n_object == '[' || n_object == ']') && (dir == '^' || dir == 'v') {
        let mut n_map = map.clone();

        let n_oh_pos = if n_object == '[' {
            (n_pos.0, n_pos.1 + 1)
        } else {
            (n_pos.0, n_pos.1 - 1)
        };

        if n_pos == step(&mut n_map, n_pos, dir) || n_oh_pos == step(&mut n_map, n_oh_pos, dir) {
            pos
        } else {
            step(map, n_pos, dir);
            step(map, n_oh_pos, dir);
            map[n_pos.0][n_pos.1] = c_object;
            map[pos.0][pos.1] = '.';

            n_pos
        }
    } else {
        panic!("Unknown object")
    }
}

fn find_position(map: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                return (i, j);
            }
        }
    }
    panic!("Object not found")
}

fn calc_gps(map: &[Vec<char>]) -> usize {
    let mut acc = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' || cell == '[' {
                acc += 100 * i + j;
            }
        }
    }
    acc
}

fn p1(input: &str) -> usize {
    let (mut map, moves) = parse(input);
    let mut pos = find_position(&map);

    for m in moves {
        pos = step(&mut map, pos, m);
    }

    calc_gps(&map)
}

fn scale_up(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();

    for row in map {
        let mut new_row = Vec::new();
        for &col in row {
            match col {
                '#' => {
                    new_row.push('#');
                    new_row.push('#')
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']')
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.')
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.')
                }
                _ => panic!("Unknown object"),
            }
        }
        new_map.push(new_row);
    }

    new_map
}

fn p2(input: &str) -> usize {
    let (map, moves) = parse(input);
    let mut new_map = scale_up(&map);
    let mut pos = find_position(&new_map);

    for m in moves {
        pos = step(&mut new_map, pos, m);
    }

    calc_gps(&new_map)
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
    fn test_p1_example1() {
        assert_eq!(p1(EXAMPLE1), 2028);
    }

    #[test]
    fn test_p1_example2() {
        assert_eq!(p1(EXAMPLE2), 10092);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(p2(EXAMPLE2), 9021);
    }
}

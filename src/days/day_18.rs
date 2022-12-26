use std::collections::{HashMap, HashSet};

const SIDES: [[isize; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

const ZERO: [isize; 3] = [0, 0, 0];


pub fn solve_1(input_lines: &[String], _verbose: bool) -> usize {
    get_field(input_lines).len()
}

pub fn solve_2(input_lines: &[String], _verbose: bool) -> usize {
    let field = get_field(input_lines);
    let mut keys = vec![];
    for x in field {
        keys.push(x.0);
    }
    keys.sort_by(|a, b| a[2].cmp(&b[2]));
    keys.sort_by(|a, b| a[1].cmp(&b[1]));
    keys.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut visited = HashSet::from([keys[0]]);
    let mut current_keys = vec![keys[0]];

    loop {
        let mut front = Vec::new();

        if front.is_empty() {
            break;
        }
        current_keys = front;
    }

    println!("{:?}", keys);
    58
}

fn line_to_coords(line: &str) -> [isize; 3] {
    let data: Vec<isize> = line.split(',').map(|x| x.parse().unwrap()).collect();
    [data[0], data[1], data[2]]
}

fn mult(n: isize, [x, y, z]: [isize; 3]) -> [isize; 3] {
    [n * x, n * y, n * z]
}

fn plus([ax, ay, az]: [isize; 3], [bx, by, bz]: [isize; 3]) -> [isize; 3] {
    [ax + bx, ay + by, az + bz]
}

fn get_field(input_lines: &[String]) -> HashMap<[isize; 3], [isize; 3]> {
    let mut field = HashMap::new();

    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let coords = mult(2, line_to_coords(line));
        for side in SIDES {
            let side_coord = plus(coords, side);
            let local_value = match field.get(&side_coord) {
                Some(existing) => plus(*existing, side),
                None => side,
            };
            field.insert(side_coord, local_value);
        }
    }

    field = field
        .iter()
        .filter(|(_, value)| **value != ZERO)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    field
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_18::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(18, None);
        assert_eq!(solve_1(&probe, true), 64);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(18, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

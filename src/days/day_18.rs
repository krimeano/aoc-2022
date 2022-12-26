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

pub fn solve_2(input_lines: &[String], verbose: bool) -> isize {
    let field = get_field(input_lines);
    let mut taken: HashSet<[isize; 3]> = HashSet::new();
    let mut surfaces: Vec<HashMap<[isize; 3], [isize; 3]>> = Vec::new();

    for _ in 0..field.len() {
        let mut keys = vec![];

        for key in field.keys() {
            if !taken.contains(key) {
                keys.push(*key);
            }
        }

        keys.sort_by(|a, b| a[2].cmp(&b[2]));
        keys.sort_by(|a, b| a[1].cmp(&b[1]));
        keys.sort_by(|a, b| a[0].cmp(&b[0]));
        // println!("keys {:?}", keys);
        let surface = find_surface(keys[0], &field, verbose);
        for key in surface.keys() {
            taken.insert(*key);
        }

        surfaces.push(surface);

        // println!("taken {:?}", taken);

        if taken.len() == field.len() {
            break;
        }
    }

    let mut total = 0;
    for surface in surfaces {
        let mut aggr = [0, 0, 0];
        for key in surface.keys() {
            aggr = plus(aggr, *key);
        }
        let size = surface.len() as isize;
        let mean = [aggr[0] / size, aggr[1] / size, aggr[2] / size];
        if verbose {
            println!("SURFACE ({},{:?}, {:?}) : {:?}", size, aggr, mean, &surface);
        }
        let mut div = 0;
        for (key, value) in surface {
            div += scalar(value, plus(key, mult(-1, mean)));
        }
        if verbose {
            println!("DIV = {:?}", div);
        }
        if div >= 0 {
            total += size;
        }
    }
    total
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

fn scalar([ax, ay, az]: [isize; 3], [bx, by, bz]: [isize; 3]) -> isize {
    ax * bx + ay * by + az * bz
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

fn find_surface(start: [isize; 3], field: &HashMap<[isize; 3], [isize; 3]>, verbose: bool) -> HashMap<[isize; 3], [isize; 3]> {
    if verbose {
        println!("FIND SURFACE FROM {:?}", start);
    }
    let mut surface = HashMap::new();
    surface.insert(start, field.get(&start).unwrap().clone());

    let mut current_keys = vec![start];

    for _ in 0..field.len() {
        if current_keys.is_empty() {
            break;
        }
        let mut front = Vec::new();
        for current in current_keys {
            let pointing = field.get(&current).unwrap();
            if verbose {
                println!("current {:?} pointing {:?}", current, pointing);
            }
            for shift in SIDES {
                if scalar(shift, *pointing) != 0 {
                    continue;
                }
                let neighbours = [
                    plus(current, plus(shift, *pointing)), // side front
                    plus(current, mult(2, shift)), // the same plane
                    plus(current, plus(shift, mult(-1, *pointing))), // behind the corner
                ];
                if verbose {
                    println!("neighbours whith shift = {:?}: {:?}", shift, neighbours);
                }
                for n in neighbours {
                    let mut projection = 0;
                    if let Some(n_pointing) = field.get(&n) {
                        projection = scalar(*pointing, *n_pointing);
                        if projection < 0 {
                            if verbose {
                                println!("opposite direction {:?} at {:?}; ignore and don't look behind the corner", n_pointing, n);
                            }
                            break;
                        }
                        if surface.contains_key(&n) {
                            continue;
                        }
                        front.push(n);
                        surface.insert(n, *n_pointing);
                        if projection > 0 {
                            break;
                        }
                    }
                }
            }
        }
        current_keys = front;
    }

    surface
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_18::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(18, None);
        assert_eq!(solve_1(&probe, false), 64);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(18, None);
        assert_eq!(solve_2(&probe, false), 58);
    }
}

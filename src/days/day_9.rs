use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rope {
    h: [i32; 2],
    t: [i32; 2],
}

impl Rope {
    pub fn new() -> Self {
        Self {
            h: [0, 0],
            t: [0, 0],
        }
    }

    pub fn th(&self) -> [i32; 2] {
        [self.h[0] - self.t[0], self.h[1] - self.t[1]]
    }

    pub fn mv(&mut self, dir: [i32; 2], verbose: bool) -> () {
        if verbose {
            println!("move {:?}", self);
            println!("th {:?}, dir {:?}", self.th(), dir);
        }

        self.h = plus(self.h, dir);
        self.t = follow(self.t, self.h);

        if verbose {
            println!("  to {:?}", self);
            println!()
        }
    }
}

const DIRS: [(char, [i32; 2]); 4] = [
    ('R', [1, 0]),
    ('L', [-1, 0]),
    ('U', [0, 1]),
    ('D', [0, -1]),
];


pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let char_dir = HashMap::from(DIRS);
    let mut rope = Rope::new();
    let mut tt = HashSet::from([[0, 0]]);
    for line in input_lines {
        let (steps, dir) = line_to_command(&char_dir, line, verbose);

        for _ in 0..steps {
            rope.mv(dir, verbose);
            tt.insert(rope.t);
        }
    }
    tt.len()
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    let char_dir = HashMap::from(DIRS);
    let mut knots = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], ];
    let mut tt = HashSet::from([[0, 0]]);
    if verbose {
        println!("{:?}", knots);
    }
    for line in input_lines {
        let (steps, dir) = line_to_command(&char_dir, line, verbose);
        for _ in 0..steps {
            knots[0] = plus(knots[0], dir);
            for ix in 1..knots.len() {
                knots[ix] = follow(knots[ix], knots[ix - 1])
            }
            if verbose {
                println!("{:?}", knots)
            }
            tt.insert(*knots.last().unwrap());
        }
    }

    tt.len()
}

fn line_to_command(char_dir: &HashMap<char, [i32; 2]>, line: &str, verbose: bool) -> (usize, [i32; 2]) {
    if line.is_empty() {
        return (0, [0, 0]);
    }

    let steps = line[2..line.len()].parse::<usize>().unwrap();

    let dir = if let Some(c) = line.chars().next() {
        if let Some(x) = char_dir.get(&c) {
            *x
        } else {
            [0, 0]
        }
    } else {
        [0, 0]
    };

    if verbose {
        println!("{:?} {:?} {:?}", line.chars().next(), steps, dir);
    }

    (steps, dir)
}

fn follow(dot: [i32; 2], aim: [i32; 2]) -> [i32; 2] { plus(dot, get_dir(dot, aim)) }

fn get_dir(dot: [i32; 2], aim: [i32; 2]) -> [i32; 2] { decrement(minus(aim, dot)) }

fn plus(a: [i32; 2], b: [i32; 2]) -> [i32; 2] { [a[0] + b[0], a[1] + b[1]] }

fn minus(a: [i32; 2], b: [i32; 2]) -> [i32; 2] { [a[0] - b[0], a[1] - b[1]] }

// fn scalar(a: [i32; 2], b: [i32; 2]) -> i32 { a[0] * b[0] + a[1] * b[1] }

fn decrement(a: [i32; 2]) -> [i32; 2] {
    if a[0].abs() <= 1 && a[1].abs() <= 1 {
        [0, 0]
    } else {
        [decrement_number(a[0]), decrement_number(a[1])]
    }
}

fn decrement_number(n: i32) -> i32 {
    if n > 1 {
        n - 1
    } else if n < -1 {
        n + 1
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_9::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(9, None);
        assert_eq!(solve_1(&probe, false), 13);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_2(&read_probe(9, None), false), 1);
        assert_eq!(solve_2(&read_probe(9, Some(1)), false), 36);
    }
}

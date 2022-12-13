use std::collections::HashSet;

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
        let projection = scalar(self.th(), dir);
        if verbose {
            println!("move {:?}", self);
            println!("th {:?}, dir {:?}, proj {:?}", self.th(), dir, projection);
        }

        if projection > 0 {
            self.t[0] = self.h[0];
            self.t[1] = self.h[1];
        }

        self.h[0] += dir[0];
        self.h[1] += dir[1];

        if verbose {
            println!("  to {:?}", self);
            println!()
        }
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let mut rope = Rope::new();
    let mut tt = HashSet::from([[0, 0]]);
    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let steps = line[2..line.len()].parse::<usize>().unwrap();
        println!("{:?} {:?}", line.chars().next(), steps);
        let dir = match line.chars().next() {
            Some('R') => [1, 0],
            Some('L') => [-1, 0],
            Some('U') => [0, 1],
            Some('D') => [0, -1],
            _ => [0, 0]
        };
        for _ in 0..steps {
            rope.mv(dir, verbose);
            tt.insert(rope.t);
        }
    }
    tt.len()
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

fn scalar(a: [i32; 2], b: [i32; 2]) -> i32 {
    a[0] * b[0] + a[1] * b[1]
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_9::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(9, None);
        assert_eq!(solve_1(&probe, true), 13);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(9, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

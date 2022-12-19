use std::fmt;

struct Op {
    op: char,
    old: bool,
    number: u64,
}

impl Op {
    pub fn new() -> Self {
        Self {
            op: '+',
            old: false,
            number: 0,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    yes: usize,
    no: usize,
    div: u64,
    op: Op,
    inspected: usize,
    s_items: Vec<u64>,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            yes: 0,
            no: 0,
            div: 1,
            op: Op::new(),
            inspected: 0,
            s_items: vec![],
        }
    }

    pub fn inspect(&self, item: u64, should_calm: bool, cap: u64) -> (usize, u64) {
        let mut level = self.worry(item, cap);
        if should_calm {
            level = self.calm(level);
        }
        let ix = if level % self.div == 0 {
            self.yes
        } else {
            self.no
        };
        (ix, level)
    }

    fn worry(&self, item: u64, cap: u64) -> u64 {
        let a = item % cap;
        let b = (if self.op.old { item } else { self.op.number }) % cap;
        if self.op.op == '*' {
            (a * b) % cap
        } else {
            (a + b) % cap
        }
    }

    fn calm(&self, worried: u64) -> u64 {
        worried / 3
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = if self.old {
            "old".to_string()
        } else {
            format!("{}", self.number)
        };
        write!(f, "old {} {}", self.op, val)
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    solve(input_lines, verbose, 20, true)
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    solve(input_lines, verbose, 10000, false)
}

pub fn solve(input_lines: &[String], verbose: bool, rounds: usize, should_calm: bool) -> usize {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input_lines);
    let size = monkeys.len();
    let mut all_items = monkeys
        .iter()
        .map(|x| x.s_items.clone())
        .collect::<Vec<Vec<u64>>>();
    if verbose {
        print_monkeys(&monkeys);
    }
    let mut cap = if should_calm { 3 } else { 1 };
    for ix in 0..size {
        cap *= monkeys[ix].div;
    }
    if verbose {
        println!("CAP = {}, CAP**2 = {}", cap, cap * cap);
    }
    if verbose {
        println!("{:?}", all_items);
    }
    for round in 1..=rounds {
        for ix in 0..size {
            let mut passes = Vec::new();
            while !all_items[ix].is_empty() {
                let item = all_items[ix].pop().unwrap();
                passes.push(monkeys[ix].inspect(item, should_calm, cap));
                monkeys[ix].inspected += 1;
            }
            while !passes.is_empty() {
                let (jy, item) = passes.pop().unwrap();
                all_items[jy].push(item);
            }
        }
        if verbose {
            println!("{}: {:?}", round, all_items);
        }
    }
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    if verbose {
        print_monkeys(&monkeys);
    }
    monkeys[0].inspected * monkeys[1].inspected
}

fn parse_monkeys(input_lines: &[String]) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_lines: Vec<String> = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            monkeys.push(parse_monkey(&monkey_lines));
            monkey_lines = Vec::new();
            continue;
        }
        monkey_lines.push(line.clone())
    }
    monkeys
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let mut monkey = Monkey::new();
    for line in lines {
        let parts = line.trim().split(": ").collect::<Vec<&str>>();
        match parts[0] {
            "Starting items" => {
                monkey.s_items = parts[1].split(", ").map(|x| x.parse().unwrap()).collect();
            }
            "Operation" => {
                let val = parts[1].split(' ').last().unwrap();
                monkey.op.op = parts[1].chars().nth(10).unwrap();
                monkey.op.old = val == "old";
                monkey.op.number = if val == "old" {
                    0
                } else {
                    val.parse().unwrap()
                };
            }
            "Test" => monkey.div = parts[1].split(' ').last().unwrap().parse().unwrap(),
            "If true" => monkey.yes = parts[1].split(' ').last().unwrap().parse().unwrap(),
            "If false" => monkey.no = parts[1].split(' ').last().unwrap().parse().unwrap(),
            _ => {}
        }
    }
    monkey
}

fn print_monkeys(monkeys: &[Monkey]) {
    for monkey in monkeys {
        println!("{:?}", monkey);
    }
    println!()
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_11::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(11, None);
        assert_eq!(solve_1(&probe, false), 10605);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(11, None);
        assert_eq!(solve_2(&probe, false), 2713310158);
    }
}

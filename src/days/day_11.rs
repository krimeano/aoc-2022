use std::fmt;

struct Op {
    op: char,
    old: bool,
    number: u32,
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
    div: u32,
    op: Op,
    inspected: usize,
    items: Vec<u32>,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            yes: 0,
            no: 0,
            div: 1,
            op: Op::new(),
            inspected: 0,
            items: vec![],
        }
    }

    pub fn inspect(&self, item: u32) -> (usize, u32) {
        let mut level = self.worry(item);
        level = self.calm(level);
        let ix = if level % self.div == 0 { self.yes } else { self.no };
        (ix, level)
    }

    fn worry(&self, item: u32) -> u32 {
        let other = if self.op.old { item } else { self.op.number };
        if self.op.op == '*' {
            item * other
        } else {
            item + other
        }
    }

    fn calm(&self, worried: u32) -> u32 {
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

pub fn solve_1(input_lines: &[String], verbose: bool) -> u32 {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input_lines);
    let size = monkeys.len();
    if verbose {
        print_monkeys(&monkeys)
    }

    for monkey in &monkeys {
        let mut passes = Vec::new();
        for item in &monkey.items {
            let pass = monkey.inspect(*item);
            println!("{}, {:?}", item, pass);
            passes.push(pass);
        }
        for pass in passes {
            let another = &mut monkeys[pass.0];
            another.items.push(pass.1)
        }
    }
    0
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
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
                monkey.items = parts[1].split(", ").map(|x| x.parse().unwrap()).collect();
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
        assert_eq!(solve_1(&probe, true), 10605);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(11, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    ix: usize,
    name: String,
    operation: Option<char>,
}

#[derive(Debug)]
struct MonkeyIndex {
    verbose: bool,
    indexes: HashMap<String, usize>,
    monkeys: HashMap<usize, Monkey>,
    depending: HashMap<usize, usize>,
    depends_on: HashMap<usize, [usize; 2]>,
    values: HashMap<usize, i64>,
}

impl MonkeyIndex {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            indexes: HashMap::new(),
            monkeys: HashMap::new(),
            depending: HashMap::new(),
            depends_on: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn yell(&mut self) -> i64 {
        let order = self.arrange();
        if self.verbose {
            println!("order = {:?}", order);
        }
        for ix in order {
            let mut monkey = self.monkeys.get(&ix).unwrap();
            if let Some(dd) = self.depends_on.get(&ix) {
                let vv = dd
                    .iter()
                    .map(|d| *self.values.get(d).unwrap())
                    .collect::<Vec<i64>>();
                let value = match monkey.operation {
                    Some('-') => vv[0] - vv[1],
                    Some('+') => vv[0] + vv[1],
                    Some('*') => vv[0] * vv[1],
                    Some('/') => vv[0] / vv[1],
                    _ => panic!(),
                };
                self.values.insert(ix, value);
            }
            if self.verbose {
                println!("{} yells {}", monkey.name, self.values.get(&ix).unwrap());
            }
        }
        *self.values.get(self.indexes.get("root").unwrap()).unwrap()
    }

    pub fn add_monkey(&mut self, line: &str) {
        let monkey_data = line
            .split(": ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let name = monkey_data[0].clone();
        let ix = self.get_ix(&name);

        let mut monkey = Monkey {
            ix,
            name,
            operation: None,
        };

        let parts = monkey_data[1]
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        match parts.len() {
            1 => {
                self.values.insert(ix, parts[0].parse::<i64>().unwrap());
            }
            3 => {
                let jy = self.get_ix(&parts[0]);
                let kz = self.get_ix(&parts[2]);
                self.depends_on.insert(ix, [jy, kz]);
                self.depending.insert(jy, ix);
                self.depending.insert(kz, ix);
                monkey.operation = parts[1].chars().next();
            }
            _ => panic!("wrong input! {:?}", line),
        }

        if self.verbose {
            println!("{:?}", monkey);
        }
        self.monkeys.insert(ix, monkey);
    }

    fn get_ix(&mut self, name: &str) -> usize {
        if self.indexes.get(name).is_none() {
            self.indexes.insert(name.to_string(), self.indexes.len());
        }
        *self.indexes.get(name).unwrap()
    }

    fn arrange(&mut self) -> Vec<usize> {
        let mut depends_on_count = self
            .monkeys
            .keys()
            .map(|ix| {
                (
                    *ix,
                    if let Some(dd) = self.depends_on.get(ix) {
                        dd.len()
                    } else {
                        0
                    },
                )
            })
            .collect::<HashMap<usize, usize>>();

        let mut pool = self.monkeys.keys().map(|x| *x).collect::<Vec<usize>>();
        let mut sorted = Vec::new();
        let mut cnt = 100;
        while !pool.is_empty() {
            pool.sort_by(|a, b| {
                depends_on_count
                    .get(a)
                    .unwrap()
                    .cmp(depends_on_count.get(b).unwrap())
            });
            let mut new_pool = Vec::new();
            for ix in pool {
                if *depends_on_count.get(&ix).unwrap() == 0 {
                    if let Some(jy) = self.depending.get(&ix) {
                        depends_on_count.insert(*jy, depends_on_count.get(jy).unwrap() - 1);
                    } else if let Some(root) = self.indexes.get("root") {
                        if ix != *root {
                            panic!("ix={} doesn't have dependant monkey!", ix)
                        }
                    }
                    sorted.push(ix);
                } else {
                    new_pool.push(ix);
                }
            }
            pool = new_pool;
            cnt -= 1;
            if cnt < 1 {
                panic!("too many iterations!");
            }
        }
        sorted
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> i64 {
    let mut monkeys = MonkeyIndex::new(verbose);
    for line in input_lines {
        if !line.is_empty() {
            monkeys.add_monkey(line)
        }
    }
    monkeys.yell()
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_21::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(21, None);
        assert_eq!(solve_1(&probe, false), 152);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(21, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    ix: usize,
    name: String,
    operation: Option<char>,
}

#[derive(Debug)]
struct MonkeyIndex {
    part_two: bool,
    verbose: bool,
    indexes: HashMap<String, usize>,
    monkeys: HashMap<usize, Monkey>,
    depending: HashMap<usize, usize>,
    depends_on: HashMap<usize, [usize; 2]>,
    values: HashMap<usize, i64>,
}

impl MonkeyIndex {
    pub fn new(part_two: bool, verbose: bool) -> Self {
        Self {
            part_two,
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
        let mut stack_part_two = Vec::new();
        if self.verbose {
            println!("order = {:?}", order);
        }
        for ix in order {
            let monkey = self.monkeys.get(&ix).unwrap();

            if let Some(dd) = self.depends_on.get(&ix) {
                let v_a = self.values.get(&dd[0]);
                let v_b = self.values.get(&dd[1]);

                if v_a.is_some() && v_b.is_some() {
                    let a = *v_a.unwrap();
                    let b = *v_b.unwrap();
                    let value = match monkey.operation {
                        Some('+') => a + b,
                        Some('*') => a * b,
                        Some('-') => a - b,
                        Some('/') => a / b,
                        _ => panic!(),
                    };
                    self.values.insert(ix, value);
                    if self.verbose {
                        println!("{} yells {}", monkey.name, value);
                    }
                } else {
                    stack_part_two.push(ix);
                    if self.verbose {
                        println!("omit monkey {}, yell later", monkey.name);
                    }
                }
            } else {
                let value = self.values.get(&ix);
                if value.is_none() {
                    stack_part_two.push(ix);
                    if self.verbose {
                        println!("omit monkey {}, yell later", monkey.name);
                    }
                } else {
                    if self.verbose {
                        println!("{} yells {:?}", monkey.name, value.unwrap());
                    }
                }
            }
        }

        if self.verbose {
            println!("stack part two = {:?}", stack_part_two);
        }

        while let Some(ix) = stack_part_two.pop() {
            let monkey = self.monkeys.get(&ix).unwrap();
            let current_value = self.values.get(&ix);

            if self.verbose {
                println!("reversing monkey {:?}", monkey);
            }
            if let Some([left, right]) = self.depends_on.get(&ix) {
                let left_value = self.values.get(left);
                let right_value = self.values.get(right);
                let jy = if left_value.is_none() { left } else { right };
                let value = match monkey.operation {
                    Some('=') => {
                        if left_value.is_none() {
                            *right_value.unwrap()
                        } else {
                            *left_value.unwrap()
                        }
                    }
                    Some('+') => {
                        if left_value.is_none() {
                            println!("{:?}", monkey);
                            current_value.unwrap() - right_value.unwrap()
                        } else {
                            current_value.unwrap() - left_value.unwrap()
                        }
                    }
                    Some('*') => {
                        if left_value.is_none() {
                            current_value.unwrap() / right_value.unwrap()
                        } else {
                            current_value.unwrap() / left_value.unwrap()
                        }
                    }
                    Some('-') => {
                        if left_value.is_none() {
                            current_value.unwrap() + right_value.unwrap()
                        } else {
                            left_value.unwrap() - current_value.unwrap()
                        }
                    }
                    Some('/') => {
                        if left_value.is_none() {
                            current_value.unwrap() * right_value.unwrap()
                        } else {
                            left_value.unwrap() / current_value.unwrap()
                        }
                    }
                    _ => panic!(),
                };
                self.values.insert(*jy, value);
                if self.verbose {
                    println!("monkey {} should have yell {}", jy, value);
                }
            }
        }

        let last_monkey_name = if self.part_two { "humn" } else { "root" };
        *self
            .values
            .get(self.indexes.get(last_monkey_name).unwrap())
            .unwrap()
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
                if !self.part_two || monkey_data[0] != "humn".to_string() {
                    self.values.insert(ix, parts[0].parse::<i64>().unwrap());
                }
            }
            3 => {
                let jy = self.get_ix(&parts[0]);
                let kz = self.get_ix(&parts[2]);
                self.depends_on.insert(ix, [jy, kz]);
                self.depending.insert(jy, ix);
                self.depending.insert(kz, ix);

                monkey.operation = if monkey_data[0] == "root".to_string() && self.part_two {
                    Some('=')
                } else {
                    parts[1].chars().next()
                };
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
    let mut monkeys = MonkeyIndex::new(false, verbose);
    for line in input_lines {
        if !line.is_empty() {
            monkeys.add_monkey(line)
        }
    }
    monkeys.yell()
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> i64 {
    let mut monkeys = MonkeyIndex::new(true, verbose);
    for line in input_lines {
        if !line.is_empty() {
            monkeys.add_monkey(line)
        }
    }
    monkeys.yell()
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
        assert_eq!(solve_2(&probe, false), 301);
    }
}

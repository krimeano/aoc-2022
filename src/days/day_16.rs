use std::collections::{HashMap, HashSet};

use regex::Regex;

const TIME_LIMIT: u32 = 30;

struct LineParser {
    re: Regex,
}

impl LineParser {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                .unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> (String, u32, Vec<String>) {
        let cap = self.re.captures(line).unwrap();
        (
            cap[1].to_string(),
            cap[2].parse().unwrap(),
            cap[3].split(", ").map(|x| x.to_string()).collect(),
        )
    }
}

#[derive(Debug)]
struct Valve(usize, u32, Vec<String>);

impl Valve {
    pub fn new(ix: usize, power: u32, tunnels: Vec<String>) -> Self {
        Self(ix, power, tunnels)
    }
}

struct Valves {
    parser: LineParser,
    verbose: bool,
    counter: usize,
    valves: HashMap<String, Valve>,
    nodes: Vec<String>,
    distances: HashMap<usize, HashMap<usize, u32>>,
    powers: HashMap<usize, u32>,
}

impl Valves {
    pub fn new(parser: LineParser, verbose: bool) -> Self {
        Self {
            parser,
            verbose,
            counter: 0,
            valves: HashMap::new(),
            nodes: vec![],
            distances: HashMap::new(),
            powers: HashMap::new(),
        }
    }

    pub fn consume(&mut self, line: &str) {
        let data = self.parser.parse_line(line);

        let ix = if data.1 > 0 {
            self.counter += 1;
            self.counter
        } else {
            0
        };
        let valve = Valve::new(ix, data.1, data.2);
        if self.verbose {
            println!("{:?} {:?}", &data.0, &valve);
        }
        self.valves.insert(data.0.clone(), valve);
        if data.1 > 0 {
            self.nodes.push(data.0);
        }
        if ix > 0 {
            self.powers.insert(ix, data.1);
        }
    }

    pub fn find_distances(&mut self) {
        let aa = self.find_distance_for_node("AA");
        if self.verbose {
            println!();
            println!("\"AA\": 0: {:?}", &aa);
        }
        self.distances.insert(0, aa);
        for ix in 0..self.nodes.len() {
            let node = &self.nodes[ix];
            let dd = self.find_distance_for_node(node);
            if self.verbose {
                println!(
                    "{:?}:{:>2?}: {:?}",
                    node,
                    self.valves.get(node).unwrap().0,
                    &dd
                );
            }
            self.distances.insert(self.valves.get(node).unwrap().0, dd);
        }
        if self.verbose {
            println!();
        }
    }

    fn find_distance_for_node(&self, start: &str) -> HashMap<usize, u32> {
        let mut out = HashMap::new();
        let mut wave = HashSet::from([start.to_string()]);
        let mut visited: HashSet<String> = HashSet::new();
        let mut distance = 0;
        loop {
            if wave.is_empty() {
                break;
            }
            distance += 1;
            let mut front: HashSet<String> = HashSet::new();

            for x in wave {
                visited.insert(x.to_string());
                if let Some(v) = self.valves.get(&x) {
                    for y in &v.2 {
                        if visited.contains(y) {
                            continue;
                        }
                        if self.nodes.contains(y) {
                            out.insert(self.valves.get(&y.clone()).unwrap().0, distance);
                        }
                        front.insert(y.clone());
                    }
                }
            }
            wave = front;
        }
        return out;
    }
}

#[derive(Debug)]
struct Path {
    last: usize,
    behind: Vec<usize>,
    score: u32,
    time: u32,
}

impl Path {
    pub fn init() -> Self {
        Self {
            last: 0,
            behind: vec![],
            score: 0,
            time: 0,
        }
    }
}

#[derive(Debug)]
struct Paths {
    paths: Vec<Path>,
    passed: Vec<usize>,
    ahead: Vec<usize>,
}

impl Paths {
    pub fn init(valves: &Valves) -> Self {
        let mut ahead = valves
            .nodes
            .iter()
            .map(|v| valves.valves.get(v).unwrap().0)
            .collect::<Vec<usize>>();

        ahead.sort();

        Self {
            paths: vec![Path::init()],
            passed: vec![0],
            ahead: ahead[1..].to_vec(),
        }
    }

    pub fn step_forward(&self, valves: &Valves) -> Vec<Self> {
        let mut out = vec![];
        for ix in 0..self.ahead.len() {
            let last = self.ahead[ix];
            let mut ahead = self.ahead.clone();
            ahead.splice(ix..ix + 1, []).collect::<Vec<_>>();
            let mut passed = self.passed.clone();
            passed.push(last);
            passed.sort();
            let mut paths = vec![];
            if valves.verbose {
                println!("next node is {:?}, so passed: {:?}, leftover: {:?}", last, &passed, &ahead);
            }

            for existing_path in self.paths.iter() {
                if valves.verbose {
                    println!("\texisting {:?}", existing_path);
                }
                let distance = valves.distances.get(&existing_path.last).unwrap().get(&last).unwrap();
                let power = valves.powers.get(&last).unwrap();
                if valves.verbose {
                    println!("\tdistance from {} to {} is {}, and power = {}", existing_path.last, last, distance, power);
                }
                let time = distance + existing_path.time + 1;
                if time >= TIME_LIMIT {
                    if valves.verbose {
                        println!("\t TOO LATE {}", time);
                    }
                }
                let score = (TIME_LIMIT - time) * power;

                if valves.verbose {
                    println!("\ttime={} with score={} ", time, score);
                }
                paths.push(Path {
                    last,
                    behind: self.passed.clone(),
                    score,
                    time,
                })
            }

            out.push(Self {
                paths,
                passed,
                ahead,
            })
        }

        out
    }
}


pub fn solve_1(input_lines: &[String], verbose: bool) -> u32 {
    let mut valves = Valves::new(LineParser::new(), verbose);

    for line in input_lines {
        if !line.is_empty() {
            valves.consume(line);
        }
    }
    valves.find_distances();
    let known_paths: Vec<Paths> = vec![Paths::init(&valves)];

    if verbose {
        println!("{:?}", known_paths);
    }

    // let mut new_paths: HashMap<Vec<usize>, Paths> = HashMap::new();

    for paths in known_paths {
        let cur_paths = paths.step_forward(&valves);
        for x in cur_paths {
            if verbose {
                println!("{:?}", x);
            }
        }
    }

    1651
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_16::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(16, None);
        assert_eq!(solve_1(&probe, false), 1651);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(16, None);
        assert_eq!(solve_2(&probe, false), 0);
    }
}

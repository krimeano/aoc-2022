use std::collections::{HashMap, HashSet};

use regex::Regex;

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
struct Valve(u32, Vec<String>);

impl Valve {
    pub fn new(power: u32, tunnels: Vec<String>) -> Self {
        Self(power, tunnels)
    }
}

struct Valves {
    parser: LineParser,
    distances: HashMap<String, HashMap<String, u32>>,
    nodes: Vec<String>,
    valves: HashMap<String, Valve>,
    verbose: bool,
}

impl Valves {
    pub fn new(parser: LineParser, verbose: bool) -> Self {
        Self {
            parser,
            distances: HashMap::new(),
            nodes: vec![],
            valves: HashMap::new(),
            verbose,
        }
    }

    pub fn consume(&mut self, line: &str) {
        let data = self.parser.parse_line(line);
        let valve = Valve::new(data.1, data.2);
        if self.verbose {
            println!("{:?} {:?}", &data.0, &valve);
        }
        self.valves.insert(data.0.clone(), valve);
        if data.1 > 0 {
            self.nodes.push(data.0);
        }
    }


    pub fn find_distances(&mut self) {
        let aa = self.find_distance_for_node("AA");
        println!("\"AA\": {:?}", &aa);
        self.distances.insert("AA".to_string(), aa);
        for ix in 0..self.nodes.len() {
            let dd = self.find_distance_for_node(&self.nodes[ix]);
            println!("{:?}: {:?}", &self.nodes[ix], &dd);
            self.distances.insert(self.nodes[ix].clone(), dd);
        }
    }

    fn find_distance_for_node(&self, start: &str) -> HashMap<String, u32> {
        let mut out = HashMap::new();
        let mut wave = HashSet::from([start.to_string()]);
        let mut visited: HashSet<String> = HashSet::new();
        let mut distance = 1; // 1 minute for valve opening
        loop {
            if wave.is_empty() {
                break;
            }
            distance += 1;
            let mut front: HashSet<String> = HashSet::new();

            for x in wave {
                visited.insert(x.to_string());
                if let Some(v) = self.valves.get(&x) {
                    for y in &v.1 {
                        if visited.contains(y) {
                            continue;
                        }
                        if self.nodes.contains(y) {
                            out.insert(y.clone(), distance);
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


pub fn solve_1(input_lines: &[String], verbose: bool) -> u32 {
    let mut valves = Valves::new(LineParser::new(), verbose);

    for line in input_lines {
        if !line.is_empty() {
            valves.consume(line);
        }
    }
    valves.find_distances();
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
        assert_eq!(solve_1(&probe, true), 1651);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(16, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

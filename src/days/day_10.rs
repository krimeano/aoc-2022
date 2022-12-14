const MEASURE_AT: [usize; 7] = [20, 60, 100, 140, 180, 220, 1000];

#[derive(Debug)]
struct Monitor {
    current_ix: usize,
    total: i32,
}

impl Monitor {
    pub fn new() -> Self {
        Self { current_ix: 0, total: 0 }
    }

    pub fn log(&mut self, cycle: usize, x: i32) {
        if cycle != MEASURE_AT[self.current_ix] {
            return;
        }
        println!("Cycle {}, x={}", cycle, x);
        self.total += x * cycle as i32;
        self.current_ix += 1;
    }
}

#[derive(Debug)]
enum State {
    Noop,
    Add,
}

#[derive(Debug)]
struct Proc {
    x: i32,
    cycle: usize,
    state: State,
    monitor: Monitor,
}

impl Proc {
    pub fn new(monitor: Monitor) -> Self {
        Self {
            x: 1,
            cycle: 0,
            state: State::Noop,
            monitor,
        }
    }

    pub fn consume(&mut self, line: &str) {
        self.process(&line[0..4]);

        if line.len() > 5 {
            self.process(&line[5..line.len()])
        }
    }

    pub fn process(&mut self, cmd: &str) {
        self.cycle += 1;
        self.monitor.log(self.cycle, self.x);
        match self.state {
            State::Noop => {
                if "addx" == cmd {
                    self.state = State::Add;
                }
            }
            State::Add => {
                self.x += cmd.parse::<i32>().unwrap();
                self.state = State::Noop;
            }
        }
    }
}

pub fn solve_1(input_lines: &[String], _verbose: bool) -> i32 {
    let monitor = Monitor::new();
    let mut proc = Proc::new(monitor);

    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        println!("{:?}", line);
        proc.consume(line);
    }
    proc.monitor.total
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_10::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(10, None);
        assert_eq!(solve_1(&probe, true), 13140);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(10, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

const MEASURE_AT: [usize; 7] = [20, 60, 100, 140, 180, 220, 1000];
const SIZE: usize = 40;

#[derive(Debug)]
struct Monitor {
    current_ix: usize,
    total: i32,
    verbose: bool,
}

impl Monitor {
    pub fn new(verbose: bool) -> Self {
        Self { current_ix: 0, total: 0, verbose }
    }

    pub fn log(&mut self, cycle: usize, x: i32) {
        if cycle != MEASURE_AT[self.current_ix] {
            return;
        }
        if self.verbose {
            println!("Cycle {}, x={}", cycle, x);
        }
        self.total += x * cycle as i32;
        self.current_ix += 1;
    }
}

#[derive(Debug)]
enum State {
    Noop,
    Add,
}

struct Proc {
    x: i32,
    cycle: usize,
    state: State,
    monitor: Monitor,
    crt: Vec<char>,
}

impl Proc {
    pub fn new(monitor: Monitor) -> Self {
        Self {
            x: 1,
            cycle: 0,
            state: State::Noop,
            monitor,
            crt: vec![],
        }
    }

    pub fn consume(&mut self, line: &str) {
        self.process(&line[0..4]);

        if line.len() > 5 {
            self.process(&line[5..line.len()])
        }
    }

    fn process(&mut self, cmd: &str) {
        self.cycle += 1;
        self.monitor.log(self.cycle, self.x);
        self.draw();
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

    fn draw(&mut self) {
        let y = ((self.cycle - 1) % SIZE) as i32;
        let c = if y > self.x - 2 && y < self.x + 2 { '#' } else { '.' };
        if self.monitor.verbose {
            println!("{:?} {:?} {:?}", y, self.x, c);
        }
        self.crt.push(c)
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> i32 {
    let proc = solve(input_lines, verbose);
    proc.monitor.total
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> u32 {
    let proc = solve(input_lines, verbose);
    if verbose {
        for x in proc.crt.chunks(SIZE) {
            println!("{}", x.iter().collect::<String>());
        }
    }

    0
}

fn solve(input_lines: &[String], verbose: bool) -> Proc {
    let monitor = Monitor::new(verbose);
    let mut proc = Proc::new(monitor);

    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        if verbose {
            println!("{:?}", line);
        }
        proc.consume(line);
    }
    proc
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_10::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(10, None);
        assert_eq!(solve_1(&probe, false), 13140);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(10, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

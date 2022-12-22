use std::cmp::{max, min};

use regex::Regex;

pub fn solve_1(input_lines: &[String], y: isize, verbose: bool) -> isize {
    let parser = LineParser::new();
    let mut data = vec![];
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        let item = parser.parse_line(line);
        if verbose {
            println!("{:?}", item);
        }
        let ([x0, y0], _, d0) = item;
        let dy = max(y, y0) - min(y, y0);

        if dy > d0 {
            if verbose {
                println!("\ttoo far: {}", dy);
            }
            continue;
        }

        let piece = [x0 - d0 + dy, x0 + d0 - dy];
        if verbose {
            println!("\tpiece = {:?}, adding to {:?}", piece, data);
        }
        data = append_segment(piece, data);
        if verbose {
            println!("\tnew segments = {:?}", data);
            println!()
        }
    }
    data.iter().map(|[x0, x1]| x1 - x0).sum()
}

pub fn solve_2(input_lines: &[String], edge: isize, verbose: bool) -> isize {
    let parser = LineParser::new();
    let mut data = vec![];
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        let item = parser.parse_line(line);
        data.push(item);
    }

    data.sort_by(|a, b| b.2.cmp(&a.2));

    let mut found: Option<[isize; 2]> = None;
    let mut counter = 0;
    for ix in 0..data.len() {
        let ([x0, y0], _, size) = data[ix];
        if verbose {
            println!("{:?} {:?}", [x0, y0], size)
        }
        let plus_size = size + 1;
        for x in max(0, x0 - plus_size)..=min(edge, x0 + plus_size) {
            let dx = max(x0, x) - min(x0, x);
            let dy = plus_size - dx;
            let yy = if dy > 0 {
                vec![y0 - dy, y0 + dy]
            } else {
                vec![y0]
            };
            for y in yy {
                if y < 0 || y > edge {
                    continue;
                }
                if verbose {
                    println!("\t{:?}", [x, y]);
                }
                let mut covered = false;
                for jy in 0..data.len() {
                    let ([x1, y1], _, distance) = data[jy];
                    if ix == jy || (x0 == x1 && y0 == y1) {
                        continue;
                    }
                    counter += 1;
                    if verbose {
                        print!("\t\t{} {:?} {}: ", counter, [x1, y1], distance);
                    }
                    let d = max(x, x1) - min(x, x1) + max(y, y1) - min(y, y1);
                    if d <= distance {
                        if verbose {
                            println!("COVERED {}", d)
                        }
                        covered = true;
                        break;
                    } else if verbose {
                        println!("TOO FAR {}", d)
                    }
                }
                if !covered {
                    if verbose {
                        println!("FOUND! {:?} in {} checks", [x, y], counter)
                    }
                    found = Some([x, y]);
                    break;
                }
            }
            if found.is_some() {
                break;
            }
        }
        if found.is_some() {
            break;
        }
    }
    if let Some([x, y]) = found {
        x * 4000000 + y
    } else {
        0
    }
}

struct LineParser {
    re: Regex,
}

impl LineParser {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^.*x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)$").unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> ([isize; 2], [isize; 2], isize) {
        let cap = self.re.captures(line).unwrap();
        let sensor = [cap[1].parse().unwrap(), cap[2].parse().unwrap()];
        let beacon = [cap[3].parse().unwrap(), cap[4].parse().unwrap()];
        let distance = max(sensor[0], beacon[0]) - min(sensor[0], beacon[0])
            + max(sensor[1], beacon[1])
            - min(sensor[1], beacon[1]);
        (sensor, beacon, distance as isize)
    }
}

fn append_segment(segment: [isize; 2], segments: Vec<[isize; 2]>) -> Vec<[isize; 2]> {
    let mut out = vec![];
    let mut a = segment;
    for b in segments {
        if a[1] < b[0] || a[0] > b[1] {
            out.push(b);
        } else {
            a = [min(a[0], b[0]), max(a[1], b[1])];
        }
    }
    out.push(a);

    out
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_15::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(15, None);
        assert_eq!(solve_1(&probe, 10, false), 26);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(15, None);
        assert_eq!(solve_2(&probe, 20, false), 56000011);
    }
}

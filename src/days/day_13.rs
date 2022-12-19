use std::cmp::Ordering;

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let mut out = 0;
    let mut ix = 1;
    let mut a: Option<&String> = None;
    let mut b: Option<&String> = None;
    for line in input_lines {
        if line.is_empty() {
            if compare_packets(&a.unwrap(), &b.unwrap(), verbose).is_lt() {
                if verbose {
                    println!("{:?} < {:?}", a.unwrap(), b.unwrap());
                    println!();
                }
                out += ix;
            } else if verbose {
                println!("{:?} >= {:?}", a.unwrap(), b.unwrap());
                println!();
            }

            a = None;
            b = None;
            ix += 1;
        } else if a == None {
            a = Some(line);
        } else if b == None {
            b = Some(line);
        } else {
            panic!();
        }
    }
    out
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    let mut lines = input_lines.iter().filter(|line| !line.is_empty()).map(|line| line.clone()).collect::<Vec<String>>();
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());
    lines.sort_by(|a, b| compare_packets(a, b, false));
    if verbose {
        println!("{:#?}", lines);
    }
    let mut out = 1;
    for (ix, x) in lines.iter().enumerate() {
        if x == "[[2]]" || x == "[[6]]" {
            out *= ix + 1;
            if verbose {
                println!("index = {}", ix + 1);
            }
        }
    }

    out
}

fn compare_packets(a: &str, b: &str, verbose: bool) -> Ordering {
    if verbose {
        println!("{} vs {}", a, b);
    }
    let is_a_int = a.chars().next().unwrap() != '[' && a.chars().last().unwrap() != ']';
    let is_b_int = b.chars().next().unwrap() != '[' && b.chars().last().unwrap() != ']';
    if is_a_int && is_b_int {
        let a8 = a.parse::<i8>().unwrap();
        let b8 = b.parse::<i8>().unwrap();
        if verbose {
            println!("{} - {} = {}", a, b, a8 - b8);
        }
        return a8.cmp(&b8);
    }
    if is_a_int && !is_b_int {
        return compare_packets(&format!("[{}]", a), b, verbose);
    }
    if !is_a_int && is_b_int {
        return compare_packets(a, &format!("[{}]", b), verbose);
    }
    let xx = parse_packet(a);
    let yy = parse_packet(b);
    if verbose {
        println!("parsed packets: {:?}, {:?}", xx, yy);
    }

    for (ix, x) in xx.iter().enumerate() {
        if let Some(y) = yy.get(ix) {
            let sub_result = compare_packets(x, y, verbose);
            if sub_result.is_ne() {
                return sub_result;
            }
        } else {
            if verbose {
                println!("{} > {}", a, b);
            }
            return Ordering::Greater;
        }
    }

    if yy.len() > xx.len() {
        if verbose {
            println!("{} < {}", a, b);
        }
        return Ordering::Less;
    }

    if verbose {
        println!("{} = {}", a, b);
    }

    Ordering::Equal
}

fn parse_packet(p: &str) -> Vec<String> {
    let mut out = vec![];
    let mut opened = 0;
    let mut cur = String::new();
    for c in p.chars() {
        if opened == 1 {
            if c == ']' {
                opened = 0;
            }

            if c == ',' || c == ']' {
                if !cur.is_empty() {
                    out.push(cur);
                }
                cur = String::new();
                continue;
            }
        }
        if c == '[' {
            opened += 1;
            if opened == 1 {
                continue;
            }
        } else if c == ']' {
            opened -= 1;
        }
        cur.push(c);
    }
    return out;
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_13::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(13, None);
        assert_eq!(solve_1(&probe, false), 13);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(13, None);
        assert_eq!(solve_2(&probe, false), 140);
    }
}

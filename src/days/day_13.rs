pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let mut out = 0;
    let mut ix = 1;
    let mut a: Option<&String> = None;
    let mut b: Option<&String> = None;
    for line in input_lines {
        if line.is_empty() {
            if compare_packets(&a.unwrap(), &b.unwrap(), verbose) < 0 {
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

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

fn compare_packets(a: &str, b: &str, verbose: bool) -> i8 {
    if verbose {
        println!("{} vs {}", a, b);
    }
    let is_a_int = a.chars().next().unwrap() != '[' && a.chars().last().unwrap() != ']';
    let is_b_int = b.chars().next().unwrap() != '[' && b.chars().last().unwrap() != ']';
    if is_a_int && is_b_int {
        let result = a.parse::<i8>().unwrap() - b.parse::<i8>().unwrap();
        if verbose {
            println!("{} - {} = {}", a, b, result);
        }
        return result;
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
            if sub_result != 0 {
                return sub_result;
            }
        } else {
            if verbose {
                println!("{} > {}", a, b);
            }
            return 1;
        }
    }

    if yy.len() > xx.len() {
        if verbose {
            println!("{} < {}", a, b);
        }
        return -1;
    }

    if verbose {
        println!("{} = {}", a, b);
    }

    0
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
        assert_eq!(solve_1(&probe, true), 13);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(13, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

use std::collections::HashSet;

const GROUP_SIZE: usize = 3;

pub fn solve_1(input_lines: &[String], verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        for x in ['a', 'm', 'z', 'A', 'M', 'Z'] {
            println!("{} = {}", x, x as u32);
        }
    }

    let mut total = 0;
    for line in input_lines {
        let size = line.len();
        if size == 0 {
            continue;
        }
        if let Some(true) = verbose {
            println!("{}, {}", line, size);
        }
        if line.len() % 2 == 1 {
            panic!("Expected even length");
        }
        let half = size / 2;
        let left = str_to_set(&line[0..half]);
        let common = compare_str_with_set(left, &line[half..size]);
        total += calculate_value(common, verbose);
    }
    total
}

pub fn solve_2(input_lines: &[String], verbose: Option<bool>) -> u32 {
    let mut ix: usize = 0;
    let mut common: Option<HashSet<char>> = None;
    let mut total = 0;
    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        if let Some(true) = verbose {
            println!("{}, {}", ix, line);
        }
        match common {
            None => common = Some(str_to_set(line)),
            Some(set) => common = Some(compare_str_with_set(set, line)),
        }
        if ix == 2 {
            match common {
                None => panic!("impossimbru!"),
                Some(set) => total += calculate_value(set, verbose),
            }
            common = None
        }
        ix = (ix + 1) % GROUP_SIZE;
    }
    total
}

fn str_to_set(line: &str) -> HashSet<char> {
    line.chars().collect()
}

fn compare_str_with_set(set: HashSet<char>, line: &str) -> HashSet<char> {
    let mut out = HashSet::new();
    for x in line.chars() {
        if set.contains(&x) {
            out.insert(x);
        }
    }
    out
}

fn calculate_value(common: HashSet<char>, verbose: Option<bool>) -> u32 {
    let mut out = 0;

    for x in common {
        let value = char_to_value(x);
        if let Some(true) = verbose {
            println!("common '{}' = {}", x, value)
        }
        out += value;
    }

    out
}

fn char_to_value(c: char) -> u32 {
    if c as u32 > 'Z' as u32 {
        1 + c as u32 - 'a' as u32
    } else {
        27 + c as u32 - 'A' as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_3::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(3, None);
        assert_eq!(solve_1(&probe, Some(false)), 157);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(3, None);
        assert_eq!(solve_2(&probe, Some(false)), 70);
    }
}

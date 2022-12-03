use std::collections::HashSet;

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
        let mut left = HashSet::new();
        for x in line[0..half].chars() {
            left.insert(x);
        }
        let mut common = HashSet::new();
        for x in line[half..size].chars() {
            if left.contains(&x) && !common.contains(&x) {
                common.insert(x);
            }
        }
        for x in common {
            let value = if x as u32 > 'Z' as u32 {
                1 + x as u32 - 'a' as u32
            } else {
                27 + x as u32 - 'A' as u32
            };
            if let Some(true) = verbose {
                println!("common '{}' = {}", x, value)
            }
            total += value;
        }
    }
    total
}

pub fn solve_2(_input_lines: &[String], _verbose: Option<bool>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_3::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(3, None);
        assert_eq!(solve_1(&probe, Some(true)), 157);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(3, None);
        assert_eq!(solve_2(&probe, Some(true)), 0);
    }
}

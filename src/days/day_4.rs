pub fn solve_1(input_lines: &[String], _verbose: Option<bool>) -> u32 {
    let mut total = 0;
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        let pair = line.split(',').map(|x| x.split('-').map(|y| y.parse().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]) {
            total += 1;
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
    use crate::days::day_4::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(4, None);
        assert_eq!(solve_1(&probe, Some(true)), 2);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(4, None);
        assert_eq!(solve_2(&probe, Some(true)), 0);
    }
}

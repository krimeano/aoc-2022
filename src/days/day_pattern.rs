pub fn solve_1(_input_lines: &[String], _verbose: Option<bool>) -> u32 {
    0
}

pub fn solve_2(_input_lines: &[String], _verbose: Option<bool>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_pattern::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(0, None);
        assert_eq!(solve_1(&probe, Some(true)), 0);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(0, None);
        assert_eq!(solve_2(&probe, Some(true)), 0);
    }
}

pub fn solve_1(input_lines: &[String], _verbose: Option<bool>) -> u32 {
    input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(',')
                .map(|y| y.split('-').map(|z| z.parse().unwrap()).collect())
                .collect::<Vec<Vec<u32>>>()
        })
        .filter(|x| {
            (x[0][0] <= x[1][0] && x[0][1] >= x[1][1]) || (x[0][0] >= x[1][0] && x[0][1] <= x[1][1])
        })
        .count() as u32
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

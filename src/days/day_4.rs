pub fn solve_1(input_lines: &[String], _verbose: bool) -> u32 {
    solve(input_lines, check_contain)
}

pub fn solve_2(input_lines: &[String], _verbose: bool) -> u32 {
    solve(input_lines, check_overlap)
}

fn solve(input_lines: &[String], check: fn(&[Vec<u8>]) -> bool) -> u32 {
    input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(',')
                .map(|y| y.split('-').map(|z| z.parse().unwrap()).collect())
                .collect::<Vec<Vec<u8>>>()
        })
        .filter(|x| check(x))
        .count() as u32
}

fn check_contain(pair: &[Vec<u8>]) -> bool {
    let a = pair[0][0];
    let b = pair[0][1];
    let c = pair[1][0];
    let d = pair[1][1];
    (a <= c && b >= d) || (a >= c && b <= d)
}

fn check_overlap(pair: &[Vec<u8>]) -> bool {
    let a = pair[0][0];
    let b = pair[0][1];
    let c = pair[1][0];
    let d = pair[1][1];
    (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b)
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_4::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(4, None);
        assert_eq!(solve_1(&probe, true), 2);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(4, None);
        assert_eq!(solve_2(&probe, true), 4);
    }
}

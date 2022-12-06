use std::collections::HashSet;

const FRAME_SIZE: usize = 4;
const FRAME_SIZE_M: usize = 14;

pub fn solve_1(input_lines: &[String], _verbose: bool) -> usize {
    solve(&input_lines[0], FRAME_SIZE)
}

pub fn solve_2(input_lines: &[String], _verbose: bool) -> usize {
    solve(&input_lines[0], FRAME_SIZE_M)
}

fn solve(input: &String, frame_size: usize) -> usize {
    let size = input.len();
    for ix in 0..size - frame_size {
        if all_unique(&input[ix..ix + frame_size]) {
            return  ix + frame_size;
        }
    }
    0
}

fn all_unique(piece: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in piece.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_6::{solve_1, solve_2};

    #[test]
    fn part_1() {
        assert_eq!(solve_1(&read_probe(6, None), true), 7);
        assert_eq!(solve_1(&read_probe(6, Some(1)), true), 5);
        assert_eq!(solve_1(&read_probe(6, Some(2)), true), 6);
        assert_eq!(solve_1(&read_probe(6, Some(3)), true), 10);
        assert_eq!(solve_1(&read_probe(6, Some(4)), true), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_2(&read_probe(6, None), true), 19);
        assert_eq!(solve_2(&read_probe(6, Some(1)), true), 23);
        assert_eq!(solve_2(&read_probe(6, Some(2)), true), 23);
        assert_eq!(solve_2(&read_probe(6, Some(3)), true), 29);
        assert_eq!(solve_2(&read_probe(6, Some(4)), true), 26);
    }
}

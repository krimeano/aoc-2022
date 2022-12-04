pub fn solve_1(input_lines: &[String], verbose: bool) -> u32 {
    // A, X - Rock = 1
    // B, Y - Paper = 2
    // C, Z - Scissors = 3
    // win = 6, tie = 3, lost = 0
    solve(input_lines, verbose, get_round_score_1)
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> u32 {
    solve(input_lines, verbose, get_round_score_2)
}

fn solve(input_lines: &[String], verbose: bool, get_round_score: fn(&str) -> u32) -> u32 {
    let mut score = 0;
    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let round_score = get_round_score(line);
        if verbose {
            println!(" = {}", round_score);
        }
        score += round_score;
    }
    score
}

fn get_round_score_1(line: &str) -> u32 {
    let win_score = match line {
        "A Y" | "B Z" | "C X" => 6,
        "A X" | "B Y" | "C Z" => 3,
        _ => 0,
    };
    let base_score = match line.chars().last() {
        Some('X') => 1,
        Some('Y') => 2,
        Some('Z') => 3,
        _ => 0,
    };
    win_score + base_score
}

fn get_round_score_2(line: &str) -> u32 {
    let win_score = match line.chars().last() {
        Some('X') => 0,
        Some('Y') => 3,
        Some('Z') => 6,
        _ => panic!("Invalid Last Char"),
    };

    let base_score = match line.chars().next() {
        Some('A') => 0,
        Some('B') => 1,
        Some('C') => 2,
        _ => panic!("Invalid First Char"),
    };

    let shift_score = match line.chars().last() {
        Some('X') => 2, // lose
        Some('Y') => 0, // draw
        Some('Z') => 1, // win
        _ => panic!("Invalid Last Char"),
    };

    win_score + (base_score + shift_score) % 3 + 1
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_2::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(2, None);
        assert_eq!(solve_1(&probe, false), 15);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(2, None);
        assert_eq!(solve_2(&probe, false), 12);
    }
}

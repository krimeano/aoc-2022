pub fn solve_1(input_lines: &[String], verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        println!("SOLVING DAY 1 PART 1");
    }
    solve(input_lines, 1)
}

pub fn solve_2(input_lines: &[String], verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        println!("SOLVING DAY 1 PART 2")
    }
    solve(input_lines, 3)
}

fn solve(input_lines: &[String], supply: usize) -> u32 {
    make_elves(input_lines)[0..supply].iter().sum()
}

fn make_elves(input_lines: &[String]) -> Vec<u32> {
    let mut elves = Vec::new();
    let mut current_elf = 0;
    for line in input_lines {
        match line.as_str() {
            "" => {
                elves.push(current_elf);
                current_elf = 0;
            }
            _ => current_elf += line.parse::<u32>().unwrap(),
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_1::{solve_1, solve_2};

    #[test]
    fn day_1_1() {
        let probe = read_probe(1, None);
        assert_eq!(solve_1(&probe, Some(false)), 24000);
    }

    #[test]
    fn day_1_2() {
        let probe = read_probe(1, None);
        assert_eq!(solve_2(&probe, Some(false)), 45000);
    }
}

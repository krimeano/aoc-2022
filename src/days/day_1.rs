use std::cmp::max;

use crate::aoc_lib::read_lines;

pub fn solve_1(file_path: String, verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        println!("SOLVING DAY 1 PART 1 from file {}", file_path)
    }
    let lines = read_lines(file_path);
    let mut current_elf = 0;
    let mut max_calories = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                max_calories = max(current_elf, max_calories);
                current_elf = 0;
            }
            _ => match line.parse::<u32>() {
                Ok(x) => current_elf += x,
                Err(e) => panic!("{}", e)
            }
        }
    }
    max_calories
}

pub fn solve_2(file_path: String, verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        println!("SOLVING DAY 1 PART 2 from file {}", file_path)
    }
    0
}


#[cfg(test)]
mod tests {
    use crate::aoc_lib::make_file_name;
    use crate::days::day_1::{solve_1, solve_2};

    #[test]
    fn day_1_1() {
        assert_eq!(solve_1(make_file_name(false, 1, None), Some(true)), 24000);
    }

    #[test]
    fn day_1_2() {
        assert_eq!(solve_2(make_file_name(false, 1, None), None), 0);
    }
}

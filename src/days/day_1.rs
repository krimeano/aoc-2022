pub fn solve_1(file_path: String, verbose: Option<bool>) -> u32 {
    if let Some(true) = verbose {
        println!("SOLVING DAY 1 PART 1 from file {}", file_path)
    }
    0
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
        assert_eq!(solve_1(make_file_name(false, 1, None), None), 24000);
    }

    #[test]
    fn day_1_2() {
        assert_eq!(solve_2(make_file_name(false, 1, None), None), 0);
    }
}

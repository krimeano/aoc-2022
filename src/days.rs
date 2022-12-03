use crate::aoc_lib::read_day;

mod day_1;

pub fn solve(day: u8, verbose: Option<bool>) {
    println!("Solving day {}", day);
    let r1 = day_1::solve_1(&read_day(1, None), verbose);
    println!("Part 1: {}", r1);
    let r2 = day_1::solve_2(&read_day(1, None), verbose);
    println!("Part 2: {}", r2);
}

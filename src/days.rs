use crate::aoc_lib::read_day;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_pattern;

pub fn solve(day: u8, verbose: bool) {
    println!("Solving day {}", day);

    match day {
        1 => {
            let r1 = day_1::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_1::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        2 => {
            let r1 = day_2::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_2::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        3 => {
            let r1 = day_3::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_3::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        4 => {
            let r1 = day_4::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_4::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        5 => {
            let r1 = day_5::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_5::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        6 => {
            let r1 = day_6::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_6::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        7 => {
            let r1 = day_7::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_7::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        _ => {
            let r1 = day_pattern::solve_1(&read_day(0, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_pattern::solve_2(&read_day(0, None), verbose);
            println!("Part 2: {}", r2);

            panic!("unknown day {}", day)
        }
    }
}

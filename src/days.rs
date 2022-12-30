use crate::aoc_lib::read_day;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_18;
mod day_2;
mod day_21;
mod day_22;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
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

        8 => {
            let r1 = day_8::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_8::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        9 => {
            let r1 = day_9::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_9::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        10 => {
            let r1 = day_10::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_10::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        11 => {
            let r1 = day_11::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_11::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        12 => {
            let r1 = day_12::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_12::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        13 => {
            let r1 = day_13::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_13::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        14 => {
            let r1 = day_14::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_14::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        15 => {
            let r1 = day_15::solve_1(&read_day(day, None), 2000000, verbose);
            println!("Part 1: {}", r1);
            let r2 = day_15::solve_2(&read_day(day, None), 4000000, verbose);
            println!("Part 2: {}", r2);
        }

        16 => {
            let r1 = day_16::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_16::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        18 => {
            let r1 = day_18::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_18::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        21 => {
            let r1 = day_21::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_21::solve_2(&read_day(day, None), verbose);
            println!("Part 2: {}", r2);
        }

        22 => {
            let r1 = day_22::solve_1(&read_day(day, None), verbose);
            println!("Part 1: {}", r1);
            let r2 = day_22::solve_2(&read_day(day, None), verbose);
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

use std::cmp::{max, min};

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    solve(input_lines, verbose, false)
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    solve(input_lines, verbose, true)
}

pub fn solve(input_lines: &[String], verbose: bool, extend: bool) -> usize {
    let lines = lines_to_coords(input_lines);
    let limits = find_limits(&lines, extend);

    if verbose {
        println!("{:?}", limits);
    }

    let mut reservoir = create_reservoir(limits, &lines, extend, verbose);

    if verbose { print_me(&reservoir); }

    let count_sand = drop_sands(&mut reservoir, limits, verbose);

    if verbose { print_me(&reservoir); }
    count_sand
}


fn lines_to_coords(input_lines: &[String]) -> Vec<Vec<[usize; 2]>> {
    input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(|piece| {
                    piece
                        .split(',')
                        .map(|coord| coord.parse().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|xx| [xx[0], xx[1]])
                .collect()
        })
        .collect()
}

fn find_limits(lines: &[Vec<[usize; 2]>], extend: bool) -> [[usize; 2]; 2] {
    let mut left = 500;
    let mut right = 500;
    let mut top = 0;
    let mut bottom = 0;
    for line in lines {
        for [x, y] in line {
            if *x < left {
                left = *x
            } else if *x > right {
                right = *x
            }
            if *y < top {
                top = *y
            } else if *y > bottom {
                bottom = *y
            }
        }
    }

    bottom += if extend { 2 } else { 1 };


    left = if extend {
        500 - min(bottom, 500)
    } else {
        left - 1
    };

    right = if extend {
        500 + min(bottom, 500)
    } else {
        right + 1
    };

    [[left, top], [right, bottom]]
}

fn create_reservoir([[x_min, y_min], [x_max, y_max]]: [[usize; 2]; 2], lines: &[Vec<[usize; 2]>], extended: bool, verbose: bool) -> Vec<Vec<char>> {
    let mut reservoir: Vec<Vec<char>> = (y_min..=y_max)
        .map(|_y| (x_min..=x_max).map(|_x| '.').collect())
        .collect();

    reservoir[0 - y_min][500 - x_min] = '+';

    for line in lines {
        let mut prev_dot: Option<[usize; 2]> = None;
        for dot in line {
            let [x_b, y_b] = *dot;

            if let Some([x_a, y_a]) = prev_dot {
                if verbose {
                    println!("draw from {:?} to {:?}", [x_a, y_a], [x_b, y_b]);
                }
                for x in min(x_a, x_b)..=max(x_a, x_b) {
                    for y in min(y_a, y_b)..=max(y_a, y_b) {
                        reservoir[y - y_min][x - x_min] = '#';
                    }
                }
            }

            prev_dot = Some(*dot);
        }
    }

    if extended {
        for x in x_min..=x_max {
            reservoir[y_max - y_min][x - x_min] = '#'
        }
    }
    reservoir
}

fn print_me(reservoir: &[Vec<char>]) {
    println!(
        "{:#?}",
        reservoir
            .iter()
            .map(|row| row.iter().cloned().collect())
            .collect::<Vec<String>>()
    );
}

fn drop_sands(reservoir: &mut [Vec<char>], [[x_min, y_min], [_x_max, y_max]]: [[usize; 2]; 2], verbose: bool) -> usize {
    let mut count_sand = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        let mut old_x = 500;
        let mut old_y = y_max;
        if verbose {
            print!("{:?}: new sand: {:?} ", count_sand + 1, [x, y]);
        }

        while y < y_max && !(y == old_y && x == old_x) {
            old_x = x;
            old_y = y;
            if reservoir[y + 1 - y_min][x - x_min] == '.' {
                // try go down
                y += 1
            } else if reservoir[y + 1 - y_min][x - 1 - x_min] == '.' {
                // try go down left
                y += 1;
                x -= 1;
            } else if reservoir[y + 1 - y_min][x + 1 - x_min] == '.' {
                // try go down right
                y += 1;
                x += 1;
            }
            if verbose {
                print!("-> [{}, {}] ", x, y);
            }
        }

        if y < y_max {
            reservoir[y - y_min][x - x_min] = 'o';
            count_sand += 1;
            if verbose {
                println!("STOPPED");
            }
        } else {
            if verbose {
                println!("FLOWS");
            }
            break;
        }

        if x == 500 && y == 0 {
            if verbose {
                println!("FULL");
            }
            break;
        }
    }

    count_sand
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_14::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(14, None);
        assert_eq!(solve_1(&probe, false), 24);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(14, None);
        assert_eq!(solve_2(&probe, true), 93);
    }
}

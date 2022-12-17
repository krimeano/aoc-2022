pub fn solve_1(input_lines: &[String], verbose: bool) -> i32 {
    let heights = get_heights(input_lines);
    let h = heights.len();
    if h == 0 {
        return 0;
    }
    let w = heights[0].len();
    if w == 0 {
        return 0;
    }
    let hw = (h * w) as i32;

    let mut steps: Vec<Vec<i32>> = (0..h).map(|_| (0..w).map(|_| 999).collect()).collect();

    let ((ix0, jy0), (ix1, jy1)) = find_points(&heights);

    steps[ix0][jy0] = 0;
    let mut current_cells = Vec::from([(ix0, jy0)]);
    if verbose {
        println!("start  at {:?}", (ix0, jy0));
        println!("finish at {:?}", (ix1, jy1));
        print_map(&heights);
        println!();
        print_map(&steps);
        println!();
    }

    for current_steps in 1..=hw {
        let mut new_cells = Vec::new();
        if verbose {
            println!("{:?}", current_cells);
        }
        for (ix_c, jy_c) in current_cells {
            if verbose {
                println!("from {:?}, {} steps", (ix_c, jy_c), current_steps - 1);
            }

            for (ix, jy) in get_neighbours(ix_c, jy_c, h, w) {
                if steps[ix][jy] <= current_steps {
                    continue;
                }
                if heights[ix][jy] - heights[ix_c][jy_c] > 1 {
                    continue;
                }

                steps[ix][jy] = current_steps;

                if ix == ix1 && jy == jy1 {
                    if verbose {
                        println!("ARRIVED! {} steps", current_steps);
                        print_map(&heights);
                        println!();
                        print_map(&steps);
                        println!();
                        println!("ARRIVED! {} steps", current_steps);
                    }
                    return current_steps;
                }
                if verbose {
                    println!("\tneighbour {:?}, {} steps", (ix, jy), current_steps);
                }

                new_cells.push((ix, jy));
            }
        }
        if verbose {
            print_map(&heights);
            println!();
            print_map(&steps);
            println!();
        }

        current_cells = new_cells;
    }
    0
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> i32 {
    0
}

fn get_heights(input_lines: &[String]) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let row = line
            .chars()
            .map(|x| match x {
                'S' => 0,
                'E' => 27,
                _ => x as i32 - 'a' as i32 + 1,
            })
            .collect();
        out.push(row);
    }
    out
}

fn find_points(heights: &[Vec<i32>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for ix in 0..heights.len() {
        for jy in 0..heights[ix].len() {
            if heights[ix][jy] == 0 {
                start = (ix, jy)
            } else if heights[ix][jy] == 27 {
                finish = (ix, jy)
            }
        }
    }
    (start, finish)
}

fn print_map(xxx: &[Vec<i32>]) {
    for xx in xxx {
        for x in xx {
            print!("{:4?}", x);
        }
        println!();
    }
}

fn get_neighbours(ix: usize, jy: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if ix > 0 {
        out.push((ix - 1, jy));
    }
    if jy > 0 {
        out.push((ix, jy - 1));
    }
    if ix + 1 < h {
        out.push((ix + 1, jy));
    }
    if jy + 1 < w {
        out.push((ix, jy + 1));
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_12::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(12, None);
        assert_eq!(solve_1(&probe, true), 31);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(12, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

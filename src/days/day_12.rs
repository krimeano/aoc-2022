#[derive(Debug)]
struct Cell {
    c: char,
    // height
    h: u8,
    // steps to reach
    s: Option<usize>,
    // came from
    f: Option<[usize; 2]>,
    // going to
    t: Option<[usize; 2]>,
}

impl Cell {
    pub fn new(c: char) -> Self {
        let h = match c {
            'S' => 1,
            'E' => 26,
            _ => c as u8 - b'a' + 1,
        };
        Self {
            c,
            h,
            s: None,
            f: None,
            t: None,
        }
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let [[ix0, jy0], [ix1, jy1]] = find_points(input_lines);
    solve_for_points([ix0, jy0], [ix1, jy1], input_lines, verbose).unwrap()
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    let [[ix0, jy0], [ix1, jy1]] = find_points(input_lines);
    let possible_starts = find_possible_starts(input_lines);
    if verbose {
        println!("{:#?}", input_lines);
        println!("{:?}", possible_starts);
    }
    let mut best_result = solve_for_points([ix0, jy0], [ix1, jy1], input_lines, verbose).unwrap();
    for [ix, jy] in possible_starts {
        if verbose {
            println!("{:=<80}", "");
            println!("Starting at {:?}", [ix, jy]);
        }
        if let Some(result) = solve_for_points([ix, jy], [ix1, jy1], input_lines, verbose) {
            if result < best_result {
                if verbose {
                    println!(
                        "{} is better than {} if start from [{}, {}]!",
                        result, best_result, ix, jy
                    )
                }
                best_result = result
            }
        }
    }
    best_result
}

fn find_points(lines: &[String]) -> [[usize; 2]; 2] {
    let mut start: Option<[usize; 2]> = None;
    let mut finish: Option<[usize; 2]> = None;
    for (ix, line) in lines.iter().enumerate() {
        for (jy, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some([ix, jy])
            } else if c == 'E' {
                finish = Some([ix, jy])
            }
        }
    }
    [start.unwrap(), finish.unwrap()]
}

fn find_possible_starts(input_lines: &[String]) -> Vec<[usize; 2]> {
    let lines: Vec<String> = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect();

    let mut out = vec![];
    let h = lines.len();
    if h == 0 {
        return out;
    }
    let w = lines[0].len();
    if w == 0 {
        return out;
    }
    for ix in 0..h {
        if let Some('a') = lines[ix].chars().next() {
            out.push([ix, 0])
        }

        if let Some('a') = lines[ix].chars().last() {
            out.push([ix, w - 1])
        }
    }

    for jy in 1..w - 1 {
        if let Some('a') = lines[0].chars().nth(jy) {
            out.push([0, jy])
        }

        if let Some('a') = lines[h - 1].chars().nth(jy) {
            out.push([h - 1, jy])
        }
    }
    out
}

fn solve_for_points(
    [ix0, jy0]: [usize; 2],
    [ix1, jy1]: [usize; 2],
    input_lines: &[String],
    verbose: bool,
) -> Option<usize> {
    if verbose {
        println!("{:#?}", input_lines);
        println!("start  at {:?}", (ix0, jy0));
        println!("finish at {:?}", (ix1, jy1));
    }
    let mut cells: Vec<Vec<Cell>> = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(Cell::new).collect())
        .collect();
    cells[ix0][jy0].s = Some(0);
    let h = cells.len();
    if h == 0 {
        return None;
    }
    let w = cells[0].len();
    if w == 0 {
        return None;
    }
    let mut current_cells = Vec::from([[ix0, jy0]]);
    let mut came = false;
    for current_steps in 1..=h * w {
        if came || current_cells.is_empty() {
            break;
        }
        let mut new_cells = Vec::new();
        if verbose {
            println!("{:?}", current_cells);
        }
        for [ix_c, jy_c] in current_cells {
            if came {
                break;
            }
            if verbose {
                println!("from {:?}, {:?}", [ix_c, jy_c], cells[ix_c][jy_c]);
            }

            for [ix, jy] in get_neighbours([ix_c, jy_c], [h, w]) {
                if came {
                    break;
                }
                if cells[ix][jy].s.is_some() || cells[ix][jy].h > cells[ix_c][jy_c].h + 1 {
                    continue;
                }
                cells[ix][jy].s = Some(current_steps);
                cells[ix][jy].f = Some([ix_c, jy_c]);
                if verbose {
                    println!("\tneighbour {:?}, {:?}", [ix, jy], cells[ix][jy]);
                }
                if ix == ix1 && jy == jy1 {
                    if verbose {
                        println!("ARRIVED! {} steps", current_steps);
                    }
                    came = true;
                }
                new_cells.push([ix, jy]);
            }
        }
        current_cells = new_cells;
    }

    if verbose {
        let mut ix_c = ix1;
        let mut jy_c = jy1;
        while cells[ix_c][jy_c].f.is_some() {
            let [ix, jy] = cells[ix_c][jy_c].f.unwrap();
            cells[ix][jy].t = Some([ix_c, jy_c]);
            ix_c = ix;
            jy_c = jy;
        }
        for ix in 0..h {
            println!("{}", input_lines[ix]);
        }
        println!();
        for ix in 0..h {
            for jy in 0..w {
                let c = if cells[ix][jy].c == 'E' {
                    'E'
                } else {
                    match cells[ix][jy].t {
                        None => '.',
                        Some([to_ix, to_jy]) => {
                            if to_ix < ix {
                                '^'
                            } else if to_ix > ix {
                                'v'
                            } else if to_jy < jy {
                                '<'
                            } else if to_jy > jy {
                                '>'
                            } else {
                                'X'
                            }
                        }
                    }
                };
                print!("{}", c);
            }
            println!();
        }
    }
    cells[ix1][jy1].s
}

fn get_neighbours([ix, jy]: [usize; 2], [h, w]: [usize; 2]) -> Vec<[usize; 2]> {
    let mut out = vec![];
    if ix > 0 {
        out.push([ix - 1, jy]);
    }
    if jy > 0 {
        out.push([ix, jy - 1]);
    }
    if ix + 1 < h {
        out.push([ix + 1, jy]);
    }
    if jy + 1 < w {
        out.push([ix, jy + 1]);
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
        assert_eq!(solve_1(&probe, false), 31);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(12, None);
        assert_eq!(solve_2(&probe, false), 29);
    }
}

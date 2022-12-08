const MAX_TREE: i8 = 9;

pub fn solve_1(input_lines: &[String], _verbose: bool) -> u32 {
    let forest: Vec<Vec<i8>> = input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| y as i8 - '0' as i8).collect())
        .collect();
    let h = forest.len();
    if h == 0 {
        return 0;
    }
    let w = forest[0].len();
    if w == 0 {
        return 0;
    }
    let mut visibile: Vec<Vec<bool>> = forest
        .iter()
        .map(|x| x.iter().map(|_| false).collect())
        .collect();
    for ih in 0..h {
        let ix = ih;

        let mut outer = -1;
        for jw in 0..w {
            let jy = jw;
            if forest[ix][jy] > outer {
                outer = forest[ix][jy];
                visibile[ix][jy] = true;
                if outer == MAX_TREE {
                    break;
                }
            }
        }

        let mut outer = -1;
        for jw in 0..w {
            let jy = w - jw - 1;
            if forest[ix][jy] > outer {
                outer = forest[ix][jy];
                visibile[ix][jy] = true;
                if outer == MAX_TREE {
                    break;
                }
            }
        }
    }

    for jw in 0..w {
        let jy = jw;

        let mut outer = -1;
        for ih in 0..h {
            let ix = ih;
            if forest[ix][jy] > outer {
                outer = forest[ix][jy];
                visibile[ix][jy] = true;
                if outer == MAX_TREE {
                    break;
                }
            }
        }

        let mut outer = -1;
        for ih in 0..h {
            let ix = h - ih - 1;
            if forest[ix][jy] > outer {
                outer = forest[ix][jy];
                visibile[ix][jy] = true;
                if outer == MAX_TREE {
                    break;
                }
            }
        }
    }

    visibile.iter().map(|row| row.iter().map(|&y| if y { 1 } else { 0 }).sum::<u32>()).sum()
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_8::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(8, None);
        assert_eq!(solve_1(&probe, true), 21);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(8, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

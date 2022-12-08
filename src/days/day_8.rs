use std::cmp::max;

#[derive(Debug)]
struct Levels {
    left: i8,
    top: i8,
    right: i8,
    bottom: i8,
}

#[derive(Debug)]
struct Tree {
    height: i8,
    levels: Levels,
}

impl Levels {
    pub fn new() -> Self {
        Self {
            left: -1,
            top: -1,
            right: -1,
            bottom: -1,
        }
    }
}

impl Tree {
    pub fn new(height: i8) -> Self {
        Self {
            height,
            levels: Levels::new(),
        }
    }

    pub fn visible(&self) -> bool {
        self.height > self.levels.left
            || self.height > self.levels.top
            || self.height > self.levels.right
            || self.height > self.levels.bottom
    }
}

pub fn solve_1(input_lines: &[String], _verbose: bool) -> usize {
    let mut forest: Vec<Vec<Tree>> = input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| Tree::new(y as i8 - '0' as i8)).collect())
        .collect();
    let h = forest.len();
    if h == 0 {
        return 0;
    }
    let w = forest[0].len();
    if w == 0 {
        return 0;
    }
    for ih in 0..h {
        for jw in 0..w {
            {
                let ix = ih;
                let jy = jw;
                let top = if ix > 0 {
                    max(forest[ix - 1][jy].height, forest[ix - 1][jy].levels.top)
                } else {
                    -1
                };
                let left = if jy > 0 {
                    max(forest[ix][jy - 1].height, forest[ix][jy - 1].levels.left)
                } else {
                    -1
                };
                let tree = &mut forest[ix][jy];
                tree.levels.left = left;
                tree.levels.top = top;
            }
            {
                let ix = h - ih - 1;
                let jy = w - jw - 1;
                let bottom = if ix < h - 1 {
                    max(forest[ix + 1][jy].height, forest[ix + 1][jy].levels.bottom)
                } else {
                    -1
                };
                let right = if jy < w - 1 {
                    max(forest[ix][jy + 1].height, forest[ix][jy + 1].levels.right)
                } else {
                    -1
                };
                let tree = &mut forest[ix][jy];
                tree.levels.bottom = bottom;
                tree.levels.right = right;
            }
        }
    }

    forest
        .iter()
        .map(|row| row.iter().filter(|&x| x.visible()).count())
        .sum::<usize>()
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

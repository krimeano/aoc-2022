use std::cmp::max;

#[derive(Debug)]
struct Levels {
    left: i8,
    top: i8,
    right: i8,
    bottom: i8,
}

#[derive(Debug)]
struct Space {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

#[derive(Debug)]
struct Tree {
    height: i8,
    levels: Levels,
    space: Space,
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

impl Space {
    pub fn new() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl Tree {
    pub fn new(height: i8) -> Self {
        Self {
            height,
            levels: Levels::new(),
            space: Space::new(),
        }
    }

    pub fn visible(&self) -> bool {
        self.height > self.levels.left
            || self.height > self.levels.top
            || self.height > self.levels.right
            || self.height > self.levels.bottom
    }

    pub fn total_space(&self) -> usize {
        self.space.left
            * self.space.top
            * self.space.right
            * self.space.bottom
    }
}

pub fn solve_1(input_lines: &[String], _verbose: bool) -> usize {
    let mut forest = input_to_forest(input_lines);
    let h = forest.len();
    if h == 0 {
        return 0;
    }
    let w = forest[0].len();
    if w == 0 {
        return 0;
    }
    for ih in 1..h - 1 {
        for jw in 1..w - 1 {
            {
                let ix = ih;
                let jy = jw;
                forest[ix][jy].levels.top = max(forest[ix - 1][jy].height, forest[ix - 1][jy].levels.top);
                forest[ix][jy].levels.left = max(forest[ix][jy - 1].height, forest[ix][jy - 1].levels.left);
            }
            {
                let ix = h - ih - 1;
                let jy = w - jw - 1;
                forest[ix][jy].levels.bottom = max(forest[ix + 1][jy].height, forest[ix + 1][jy].levels.bottom);
                forest[ix][jy].levels.right = max(forest[ix][jy + 1].height, forest[ix][jy + 1].levels.right);
            }
        }
    }

    forest
        .iter()
        .map(|row| row.iter().filter(|&x| x.visible()).count())
        .sum::<usize>()
}

pub fn solve_2(input_lines: &[String], _verbose: bool) -> usize {
    let mut forest = input_to_forest(input_lines);
    let h = forest.len();
    if h < 3 {
        return 0;
    }
    let w = forest[0].len();
    if w < 3 {
        return 0;
    }
    let mut best = 0;
    for ih in 1..h - 1 {
        for jw in 1..w - 1 {
            let height = forest[ih][jw].height;
            {
                let mut kx = ih;
                while kx > 0 {
                    kx -= 1;
                    if forest[kx][jw].height >= height {
                        break;
                    }
                }
                forest[ih][jw].space.top = ih - kx;
            }
            {
                let mut ky = jw;
                while ky > 0 {
                    ky -= 1;
                    if forest[ih][ky].height >= height {
                        break;
                    }
                }
                forest[ih][jw].space.left = jw - ky;
            }
            {
                let mut kx = ih;
                while kx < h-1 {
                    kx += 1;
                    if forest[kx][jw].height >= height {
                        break;
                    }
                }
                forest[ih][jw].space.bottom = kx - ih;
            }
            {
                let mut ky = jw;
                while ky < w-1 {
                    ky += 1;
                    if forest[ih][ky].height >= height {
                        break;
                    }
                }
                forest[ih][jw].space.right = ky - jw;
            }
            best = max(best, forest[ih][jw].total_space());
        }
    }
    best
}

fn input_to_forest(input_lines: &[String]) -> Vec<Vec<Tree>> {
    input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| Tree::new(y as i8 - '0' as i8)).collect())
        .collect()
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
        assert_eq!(solve_2(&probe, true), 8);
    }
}

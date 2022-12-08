use std::cmp::{max, min};

#[derive(Debug)]
struct Seen {
    top: isize,
    left: isize,
    bottom: isize,
    right: isize,
}

#[derive(Debug)]
struct Tree {
    height: i8,
    forest_size: (usize, usize),
    seen: Seen,
}

impl Seen {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            top: -1,
            left: -1,
            bottom: h as isize + 1,
            right: w as isize + 1,
        }
    }
}

impl Tree {
    pub fn new(height: i8, h: usize, w: usize) -> Self {
        Self {
            height,
            forest_size: (h, w),
            seen: Seen::new(h, w),
        }
    }

    pub fn visible(&self) -> bool {
        let xm = self.forest_size.0 as isize;
        let ym = self.forest_size.1 as isize;
        self.seen.top < 0
            || self.seen.left < 0
            || self.seen.bottom == xm
            || self.seen.right == ym
    }

    pub fn total_space(&self, ix: usize, jy: usize) -> usize {
        if ix < 1 || jy < 1 || ix > self.forest_size.0 - 2 || jy > self.forest_size.1 - 2 {
            return 0;
        }
        let top = ix - max(self.seen.top, 0) as usize;
        let left = jy - max(self.seen.left, 0) as usize;
        let bottom = min(self.seen.bottom as usize, self.forest_size.0 - 1) - ix;
        let right = min(self.seen.right as usize, self.forest_size.1 - 1) - jy;

        top * left * bottom * right
    }
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let forest = caclulate_forest(input_lines, verbose);

    forest
        .iter()
        .map(|row| row.iter().filter(|&x| x.visible()).count())
        .sum::<usize>()
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> usize {
    let forest = caclulate_forest(input_lines, verbose);

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
            best = max(best, forest[ih][jw].total_space(ih, jw));
        }
    }
    best
}

fn caclulate_forest(input_lines: &[String], verbose: bool) -> Vec<Vec<Tree>> {
    let mut forest = input_to_forest(input_lines);

    let h = forest.len();
    let w = if h > 0 { forest[0].len() } else { 0 };

    if h < 3 || w < 3 {
        return forest;
    }

    for ih in 1..h - 1 {
        for jw in 1..w - 1 {
            let (top, _) = calculate_visible_end(&forest, (ih, jw), (-1, 0), verbose);
            let (_, left) = calculate_visible_end(&forest, (ih, jw), (0, -1), verbose);
            let (bottom, _) = calculate_visible_end(&forest, (ih, jw), (1, 0), verbose);
            let (_, right) = calculate_visible_end(&forest, (ih, jw), (0, 1), verbose);
            forest[ih][jw].seen.top = top;
            forest[ih][jw].seen.left = left;
            forest[ih][jw].seen.right = right;
            forest[ih][jw].seen.bottom = bottom;
            if verbose {
                println!("{}, {}: {:?}", ih, jw, forest[ih][jw])
            }
        }
    }

    forest
}

fn input_to_forest(input_lines: &[String]) -> Vec<Vec<Tree>> {
    let lines: Vec<String> = input_lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect();

    let h = lines.len();
    let w = if h > 0 { lines[0].len() } else { 0 };

    lines
        .iter()
        .map(|x| x.chars().map(|y| Tree::new(y as i8 - '0' as i8, h, w)).collect())
        .collect()
}

fn calculate_visible_end(forest: &[Vec<Tree>], start: (usize, usize), ort: (isize, isize), verbose: bool) -> (isize, isize) {
    let h = forest.len() as isize;
    let mut ix = start.0 as isize;
    let mut jy = start.1 as isize;
    if h == 0 {
        return (ix, jy);
    }
    let w = forest[0].len() as isize;
    if w == 0 {
        return (ix, jy);
    }
    if ort.0 == 0 && ort.1 == 0 {
        return (ix, jy);
    }

    let height = forest[start.0][start.1].height;
    let mut stuck = false;

    if verbose {
        println!("TREE HEIGHT = {} at {}, {}", height, ix, jy);
    }
    while ix > 0 && jy > 0 && (ix < h - 1) && (jy < w - 1) {
        ix += ort.0;
        jy += ort.1;
        let current_height = forest[ix as usize][jy as usize].height;
        if verbose {
            println!("\t HEIGHT = {} at {}, {}", current_height, ix, jy);
        }
        if current_height >= height {
            stuck = true;
            if verbose {
                println!("\t\tSTUCK!");
            }
            break;
        }
    }

    if !stuck {
        if verbose {
            println!("\t\tFREEDOM!");
        }
        ix += ort.0;
        jy += ort.1;
    }
    (ix, jy)
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_8::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(8, None);
        assert_eq!(solve_1(&probe, false), 21);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(8, None);
        assert_eq!(solve_2(&probe, false), 8);
    }
}

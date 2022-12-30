use std::cmp::max;

pub fn solve_1(input_lines: &[String], verbose: bool) -> usize {
    let (map, path) = read_input(input_lines, verbose);
    let mut cursor = ([0, 0], 0);

    if verbose {
        println!("CURSOR INIT {:?}", cursor);
    }

    for jy in 0..map[0].len() {
        if map[0][jy].is_some() {
            cursor.0[1] = jy;
            break;
        }
    }
    if verbose {
        println!("CURSOR START {:?}", cursor);
    }
    for step in path {
        if verbose {
            println!("{:?}", &step);
        }
        match step {
            Step::Turn(dir) => {
                cursor.1 = (cursor.1 + if dir == 'R' { 1 } else { 3 }) % 4;
            }
            Step::Go(distance) => {
                for _ in 0..distance {
                    let go_to = map[cursor.0[0]][cursor.0[1]].unwrap()[cursor.1];
                    if cursor.1 % 2 == 0 {
                        // right or left
                        cursor.0[1] = go_to
                    } else {
                        // up or down
                        cursor.0[0] = go_to
                    }
                }
            }
        }
        if verbose {
            println!("CURSOR AT {:?}", cursor);
        }
    }
    1000 * (cursor.0[0] + 1) + 4 * (cursor.0[1] + 1) + cursor.1
}

pub fn solve_2(_input_lines: &[String], _verbose: bool) -> u32 {
    0
}

fn read_input(lines: &[String], verbose: bool) -> (Vec<Vec<Option<[usize; 4]>>>, Vec<Step>) {
    let mut state = ReadState::Map;
    let mut raw_data: Vec<String> = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut path = vec![];
    for line in lines {
        if line.is_empty() {
            state = ReadState::Path;
            continue;
        }
        match state {
            ReadState::Map => {
                width = max(width, line.len());
                height += 1;
                raw_data.push(line.clone());
            }
            ReadState::Path => path = read_path(line),
        }
    }
    if verbose {
        println!("PATH ({})= {:?}", path.len(), &path);
        println!("Map size h x w = {} x {}", height, width);

        for row in raw_data.iter() {
            println!("{:?}", row)
        }
    }

    let mut edges_width = vec![];
    let mut edges_height = vec![];
    for ix in 0..height {
        let prev_row = raw_data.get((ix + height - 1) % height).unwrap().clone();
        let row = raw_data.get(ix).unwrap().clone();
        let mut edges = [0, width];
        for jy in 0..width {
            if edges_height.len() <= jy {
                edges_height.push([0, height]);
            }
            let prev_up = prev_row.chars().nth(jy);
            let prev_left = row.chars().nth((jy + width - 1) % width);
            let cur = row.chars().nth(jy);
            if cur.is_none() || cur == Some(' ') {
                if prev_left.is_some() && prev_left != Some(' ') {
                    edges[1] = if jy == 0 { width } else { jy };
                }
                if prev_up.is_some() && prev_up != Some(' ') {
                    edges_height[jy][1] = if ix == 0 { height } else { ix };
                }
            } else {
                if prev_left.is_none() || prev_left == Some(' ') {
                    edges[0] = jy;
                }
                if prev_up.is_none() || prev_up == Some(' ') {
                    edges_height[jy][0] = ix;
                }
            }
        }
        // println!("{:?}", &edges);
        edges_width.push(edges);
    }
    // println!("{:?}", &edges_height);

    let mut can_go: Vec<Vec<Option<[usize; 4]>>> = (0..height)
        .map(|_| (0..width).map(|_| None).collect())
        .collect();

    for ix in 0..height {
        for jy in edges_width[ix][0]..edges_width[ix][1] {
            let c = raw_data.get(ix).unwrap().chars().nth(jy).unwrap();
            if c == '#' {
                continue;
            }
            let j_left = if jy == edges_width[ix][0] {
                edges_width[ix][1] - 1
            } else {
                jy - 1
            };
            let j_right = if jy == edges_width[ix][1] - 1 {
                edges_width[ix][0]
            } else {
                jy + 1
            };
            let i_up = if ix == edges_height[jy][0] {
                edges_height[jy][1] - 1
            } else {
                ix - 1
            };
            let i_down = if ix == edges_height[jy][1] - 1 {
                edges_height[jy][0]
            } else {
                ix + 1
            };

            let right = raw_data.get(ix).unwrap().chars().nth(j_right).unwrap();
            let down = raw_data.get(i_down).unwrap().chars().nth(jy).unwrap();
            let left = raw_data.get(ix).unwrap().chars().nth(j_left).unwrap();
            let up = raw_data.get(i_up).unwrap().chars().nth(jy).unwrap();

            let item = Some([
                if right == '.' { j_right } else { jy },
                if down == '.' { i_down } else { ix },
                if left == '.' { j_left } else { jy },
                if up == '.' { i_up } else { ix },
            ]);

            // println!("{}, {} = {}: >{} v{} <{} ^{}: {:?}", ix, jy, c, right, down, left, up, &item);
            can_go[ix][jy] = item;
        }
    }
    (can_go, path)
}

enum ReadState {
    Map,
    Path,
}

fn read_path(line: &str) -> Vec<Step> {
    let mut out = vec![];
    let mut number = String::new();
    for c in line.chars() {
        if c == 'R' || c == 'L' {
            if !number.is_empty() {
                out.push(Step::Go(number.parse().unwrap()));
                number = String::new();
            }
            out.push(Step::Turn(c))
        } else {
            number.push(c);
        }
    }
    if !number.is_empty() {
        out.push(Step::Go(number.parse().unwrap()))
    }
    out
}

#[derive(Debug)]
enum Step {
    Go(usize),
    Turn(char),
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_22::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(22, None);
        assert_eq!(solve_1(&probe, false), 6032);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(22, None);
        assert_eq!(solve_2(&probe, true), 0);
    }
}

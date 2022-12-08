use std::collections::HashMap;

const MAX_SIZE: u32 = 100000;
const DISK_SPACE: u32 = 70000000;
const DEMAND_SPACE: u32 = 30000000;

pub fn solve_1(input_lines: &[String], verbose: bool) -> u32 {
    scan_files(input_lines, verbose)
        .iter()
        .filter(|(_, &size)| size <= MAX_SIZE)
        .map(|(_, &size)| size)
        .sum()
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> u32 {
    let mut sizes: Vec<u32> = scan_files(input_lines, verbose)
        .iter()
        .map(|(_, &size)| size)
        .collect();
    sizes.sort();
    let total_size = sizes[sizes.len() - 1];
    for size in sizes {
        if size + DISK_SPACE > DEMAND_SPACE + total_size {
            return size;
        }
    }
    0
}

fn scan_files(input_lines: &[String], verbose: bool) -> HashMap<Vec<&str>, u32> {
    let mut cur_dir = vec![""];
    let mut dir_sizes = HashMap::from([(vec![""], 0)]);
    let mut file_sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let arr: Vec<&str> = line.split(' ').collect();
        match arr[0].chars().next().unwrap() {
            '$' => {
                if verbose {
                    println!("HANDLE COMMAND {:?}", &arr);
                }
                if arr[1] == "cd" {
                    match arr[2] {
                        "/" => cur_dir = vec![""],
                        ".." => {
                            cur_dir.pop();
                        }
                        _ => cur_dir.push(arr[2]),
                    }
                    if verbose {
                        println!("CHANGED DIRECTORY TO {:?}", cur_dir.join("/"));
                    }
                }
            }
            'd' => {
                let mut dir_path = cur_dir.clone();
                dir_path.push(arr[1]);
                if verbose {
                    println!("HANDLE DIR, {:?}", dir_path.join("/"));
                }
                dir_sizes.entry(dir_path.clone()).or_insert(0);
            }
            '0'..='9' => {
                let mut file_path = cur_dir.clone();
                file_path.push(arr[1]);
                let size: u32 = arr[0].parse().unwrap();
                if verbose {
                    println!("HANDLE FILE {:?}, {:?}", size, file_path.join("/"));
                }
                file_sizes.entry(file_path.clone()).or_insert(size);
            }
            _ => {
                panic!("UNKNOWN LINE {:?}", arr)
            }
        }
    }
    for entry in file_sizes.iter() {
        let mut parent_dir = entry.0.clone();
        parent_dir.pop();
        while !parent_dir.is_empty() {
            let mut size = *dir_sizes.get(&parent_dir.clone()).unwrap();
            size += entry.1;
            dir_sizes.insert(parent_dir.clone(), size);
            parent_dir.pop();
        }
    }
    dir_sizes
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_7::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(7, None);
        assert_eq!(solve_1(&probe, false), 95437);
    }

    #[test]
    fn part_2() {
        let probe = read_probe(7, None);
        assert_eq!(solve_2(&probe, false), 24933642);
    }
}

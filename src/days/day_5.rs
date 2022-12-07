#[derive(Debug)]
enum ReadMode {
    Crates,
    Commands,
}

pub fn solve_1(input_lines: &[String], verbose: bool) -> String {
    solve(input_lines, verbose, perform_command_1)
}

pub fn solve_2(input_lines: &[String], verbose: bool) -> String {
    solve(input_lines, verbose, perform_command_2)
}

pub fn solve(
    input_lines: &[String],
    verbose: bool,
    perform_command: fn(Vec<usize>, Vec<String>) -> Vec<String>,
) -> String {
    let mut mode: ReadMode = ReadMode::Crates;
    let mut crates_lines: Vec<&String> = Vec::new();
    let mut crates: Vec<String> = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            match mode {
                ReadMode::Crates => {
                    mode = ReadMode::Commands;
                    while !crates_lines.is_empty() {
                        if let Some(layer) = crates_lines.pop() {
                            if crates.is_empty() {
                                for _ in 0..layer.len() {
                                    crates.push("".to_string());
                                }
                            }
                            for (ix, x) in layer.chars().enumerate() {
                                crates[ix].push(x)
                            }
                        }
                    }
                    crates = crates
                        .iter()
                        .filter(|x| !matches!(x.chars().next(), Some(' ')))
                        .map(|x| x[1..].trim().to_string())
                        .collect();
                    if verbose {
                        println!("{:?}", crates);
                    }
                }
                _ => {}
            }
            continue;
        }
        match mode {
            ReadMode::Crates => crates_lines.push(line),
            ReadMode::Commands => {
                let cmd: Vec<usize> = line
                    .split(' ')
                    .enumerate()
                    .filter(|(ix, _)| ix % 2 == 1)
                    .map(|(_, x)| x.parse().unwrap())
                    .collect();
                if verbose {
                    println!("{:?}", cmd);
                }

                crates = perform_command(cmd, crates);

                if verbose {
                    println!("{:?}", crates)
                }
            }
        }
    }
    let mut out = String::new();
    for c in crates {
        out.push(c.chars().last().unwrap());
    }
    out
}

pub fn perform_command_1(cmd: Vec<usize>, mut crates: Vec<String>) -> Vec<String> {
    for _ in 0..cmd[0] {
        let crane = crates[cmd[1] - 1].pop().unwrap();
        crates[cmd[2] - 1].push(crane);
    }
    crates
}

pub fn perform_command_2(cmd: Vec<usize>, mut crates: Vec<String>) -> Vec<String> {
    let size = crates[cmd[1] - 1].len();
    let crane = crates[cmd[1] - 1][size - cmd[0]..].to_string();
    crates[cmd[1] - 1] = crates[cmd[1] - 1][..size - cmd[0]].to_string();
    crates[cmd[2] - 1] = crates[cmd[2] - 1][..].to_owned() + &crane[..];
    crates
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib::read_probe;
    use crate::days::day_5::{solve_1, solve_2};

    #[test]
    fn part_1() {
        let probe = read_probe(5, None);
        assert_eq!(solve_1(&probe, false), "CMZ");
    }

    #[test]
    fn part_2() {
        let probe = read_probe(5, None);
        assert_eq!(solve_2(&probe, false), "MCD");
    }
}

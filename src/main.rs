mod run_params;
mod days;
mod aoc_lib;

fn main() {
    let args = run_params::read();
    days::solve(args.day, Some(args.verbose));
}

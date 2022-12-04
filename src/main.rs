mod aoc_lib;
mod days;
mod run_params;

fn main() {
    let args = run_params::read();
    days::solve(args.day, args.verbose);
}

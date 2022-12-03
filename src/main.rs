mod run_params;
mod days;

fn main() {
    let args = run_params::read();
    days::solve(args.day, Some(args.verbose));
}

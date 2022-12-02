mod run_params;

fn main() {
    let args = run_params::read();
    println!("Day {}, Verbose: {}", args.day, args.verbose);
}

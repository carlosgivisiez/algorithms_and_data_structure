use std::env;
use std::process;

use algorithms_and_data_structure::Config;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = algorithms_and_data_structure::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

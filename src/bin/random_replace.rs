use algorithms_and_data_structure::{RandomReplace, ReplaceConfig};
use std::io;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        println!("{}", buffer.random_replace(ReplaceConfig::default()));

        buffer.clear();
    }
}

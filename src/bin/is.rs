use algorithms_and_data_structure::Is;
use std::io;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        println!(
            "{} {} {} {}",
            buffer.is_all_vowel(),
            buffer.is_all_consonant(),
            buffer.is_integer(),
            buffer.is_float(None)
        );

        buffer.clear()
    }
}

use std::io;
use algorithms_and_data_structure::algorithms::palindrome::Palindrome;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        if buffer.is_palindrome() {
            println!("YES");
        } else {
            println!("NO");
        }

        buffer.clear();
    }
}
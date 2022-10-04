use std::io;
use algorithms_and_data_structure::algorithms::caesar_cipher::CaesarCipher;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        let cipher = buffer.cipher(3);

        println!("{cipher}");

        buffer.clear();
    }
}
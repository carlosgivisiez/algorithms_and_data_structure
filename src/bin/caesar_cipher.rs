use algorithms_and_data_structure::CaesarCipher;
use std::io;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        let cipher = buffer.cipher(3);

        println!("{cipher}");

        buffer.clear();
    }
}

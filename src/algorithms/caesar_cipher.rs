pub trait CaesarCipher {
    fn cipher(&self, shift: u8) -> String;
}

impl CaesarCipher for String {
    fn cipher(&self, shift: u8) -> String {
        self.chars()
        .map(|c| (c as u8).wrapping_add(shift) as char)
        .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::CaesarCipher;
    
    #[test]
    fn different_shift_values() {
        assert_eq!(String::from("ab").cipher(0), "ab");
        assert_eq!(String::from("ab").cipher(1), "bc");
        assert_eq!(String::from("ab").cipher(2), "cd");
    }

    #[test]
    fn general_texts() {
        assert_eq!(String::from("").cipher(1), "");
        assert_eq!(String::from("ab").cipher(1), "bc");
        assert_eq!(String::from("ab ab").cipher(1), "bc!bc");
        assert_eq!(String::from("ab@ÿ").cipher(1), format!("bcA{}", 0 as char));
    }
}
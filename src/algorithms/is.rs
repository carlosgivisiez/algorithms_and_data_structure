pub trait Is {
    fn is_all_vowel(&self) -> bool;
    fn is_all_consonant(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_float(&self, separator: Option<char>) -> bool;
}

impl Is for String {
    fn is_all_vowel(&self) -> bool {
        normalize(self).chars().all(is_vowel)
    }

    fn is_all_consonant(&self) -> bool {
        normalize(self).chars().all(is_consonant)
    }

    fn is_integer(&self) -> bool {
        normalize(self).chars().all(is_digit)
    }

    fn is_float(&self, custom_separator: Option<char>) -> bool {
        let mut separator_read = false;
        let separator = match custom_separator {
            Some(s) => s,
            None => '.',
        };

        normalize(self).chars().all(|c| {
            if is_digit(c) {
                true
            } else if c == separator && !separator_read {
                separator_read = true;
                true
            } else {
                false
            }
        })
    }
}

fn normalize(text: &str) -> String {
    text.trim().to_lowercase()
}

fn is_vowel(c: char) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
        true
    } else {
        false
    }
}

fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

fn is_digit(c: char) -> bool {
    let code = c as u8;

    if '0' as u8 <= code && code <= '9' as u8 {
        true
    } else {
        false
    }
}

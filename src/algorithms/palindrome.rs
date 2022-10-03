pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        let mut temp = self.clone().to_lowercase();

        temp.retain(|c| c.is_alphanumeric());

        let mut chars = temp.trim().chars();
        
        while let (Some(from_front), Some(from_back)) = (chars.next(), chars.next_back()) {
            if from_front != from_back {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Palindrome;
    
    #[test]
    fn some_palindromes() {
        assert!(String::from("tattarrattat").is_palindrome());
        assert!(String::from("saippuakivikauppias").is_palindrome());
        assert!(String::from("02/02/2020").is_palindrome());
        assert!(String::from("1/11/11 11:11").is_palindrome());
        assert!(String::from("A man, a plan, a canal - Panama").is_palindrome());
    }

    #[test]
    fn not_palindromes() {
        assert!(!String::from("not a palindrome").is_palindrome());
        assert!(!String::from("also ab osla").is_palindrome());
        assert!(!String::from("ab").is_palindrome());
        assert!(!String::from("ababc").is_palindrome());
        assert!(!String::from("a b a b not").is_palindrome());
    }
}
pub trait Vowel {
    fn is_vowel(&self) -> bool;
}

impl Vowel for char {
    fn is_vowel(&self) -> bool {
        match self {
            'a' => true,
            'A' => true,
            'e' => true,
            'E' => true,
            'i' => true,
            'I' => true,
            'o' => true,
            'O' => true,
            'u' => true,
            'U' => true,
            _ => false,
        }
    }
}

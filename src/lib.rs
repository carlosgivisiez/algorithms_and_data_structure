mod algorithms;

pub use algorithms::caesar_cipher::CaesarCipher;
pub use algorithms::palindrome::Palindrome;
pub use algorithms::random_replace::{
    builder::RandomReplaceBuilder, simple::RandomReplace, ReplaceConfig,
};

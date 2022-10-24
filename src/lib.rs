mod algorithms;

pub use algorithms::boolean_expression::BooleanExpression;
pub use algorithms::caesar_cipher::CaesarCipher;
pub use algorithms::is::Is;
pub use algorithms::palindrome::Palindrome;
pub use algorithms::random_replace::{
    builder::RandomReplaceBuilder, simple::RandomReplace, ReplaceConfig,
};

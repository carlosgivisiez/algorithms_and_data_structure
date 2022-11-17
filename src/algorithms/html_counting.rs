use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

use crate::algorithms::vowel::Vowel;

pub struct MultiCounting {
    vowels: HashMap<char, u32>,
    consonants: HashMap<char, u32>,
    custom_word: HashMap<String, u32>,
}

impl MultiCounting {
    fn new() -> Self {
        MultiCounting {
            vowels: HashMap::new(),
            consonants: HashMap::new(),
            custom_word: HashMap::new(),
        }
    }
}

pub trait Counting {
    fn count(&self, custom_words: Vec<String>) -> MultiCounting;
}

impl Counting for String {
    fn count(&self, custom_words: Vec<String>) -> MultiCounting {
        let mut result = MultiCounting::new();
        let mut buffer = String::new();

        for c in self.chars() {
            if c.is_vowel() {
                *result.vowels.entry(c).or_insert(0) += 1;
            } else {
                *result.consonants.entry(c).or_insert(0) += 1;
            }

            increment_buffer(&mut buffer, &c, &result);
        }

        result
    }
}

fn increment_buffer(buffer: &mut String, new_char: &char, result: &MultiCounting) {
    for e in result.custom_word.keys() {
        let new_buffer = format!("{}{}", buffer, new_char);

        if e.partial_cmp(new_buffer) {
            buffer.push(*new_char);

            break;
        }
    }
}

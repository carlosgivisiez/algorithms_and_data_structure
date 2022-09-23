use std::{error::Error, io};

pub fn execute() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        if buffer.is_palindrome() {
            println!("YES");
        } else {
            println!("NO");
        }

        buffer.clear();
    }

    Ok(())
}

trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        let mut temp = self.clone();

        temp.retain(|c| !c.is_whitespace());

        let mut chars = temp.trim().chars();

        while let (Some(from_front), Some(from_back)) = (chars.next(), chars.next_back()) {
            if from_front != from_back {
                return false;
            }
        }

        true
    }
}
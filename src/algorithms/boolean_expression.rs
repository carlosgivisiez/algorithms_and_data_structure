use std::{borrow::BorrowMut, collections::HashMap, iter::Peekable, str::Chars};

type ExpressionResult = Result<bool, String>;

pub struct BooleanExpression {
    exp: String,
    identifier_table: HashMap<String, bool>,
}

impl BooleanExpression {
    pub fn build(exp: String, identifier_table: HashMap<String, bool>) -> Result<Self, String> {
        if exp.is_empty() {
            return Err("Expression cannot be empty".to_owned());
        }

        for entry in identifier_table.iter() {
            if entry.0.chars().any(|c| !c.is_alphabetic()) {
                return Err(format!("Identifier {} invalid", entry.0));
            }
        }

        Ok(BooleanExpression {
            exp,
            identifier_table,
        })
    }

    pub fn evaluate(self) -> ExpressionResult {
        self.next_token(self.exp.chars().peekable().borrow_mut())
    }

    fn next_token(&self, iterator: &mut Peekable<Chars>) -> ExpressionResult {
        skip_whitespaces(iterator);

        let mut token = String::from("");

        while let Some(c) = iterator.peek() {
            if !c.is_alphabetic() {
                break;
            }

            token = format!("{}{}", token, c);

            iterator.next();
        }

        if token == "and" {
            return self.and(iterator);
        } else if token == "or" {
            return self.or(iterator);
        } else if token == "not" {
            return self.not(iterator);
        } else {
            return self.identifier(token);
        }
    }

    fn and(&self, iterator: &mut Peekable<Chars>) -> ExpressionResult {
        expect(iterator, '(')?;

        let left_token = self.next_token(iterator)?;

        expect(iterator, ',')?;

        let right_token = self.next_token(iterator)?;

        expect(iterator, ')')?;

        Ok(left_token && right_token)
    }

    fn or(&self, iterator: &mut Peekable<Chars>) -> ExpressionResult {
        expect(iterator, '(')?;

        let left_token = self.next_token(iterator)?;

        expect(iterator, ',')?;

        let right_token = self.next_token(iterator)?;

        expect(iterator, ')')?;

        Ok(left_token || right_token)
    }

    fn not(&self, iterator: &mut Peekable<Chars>) -> ExpressionResult {
        expect(iterator, '(')?;

        let token = self.next_token(iterator)?;

        expect(iterator, ')')?;

        Ok(!token)
    }

    fn identifier(&self, name: String) -> ExpressionResult {
        if let Some(a) = self.identifier_table.get(&name) {
            Ok(a.to_owned())
        } else {
            Err(format!("Expected identifier, received '{}' instead", name))
        }
    }
}

fn skip_whitespaces(iterator: &mut Peekable<Chars>) {
    while let Some(c) = iterator.peek() {
        if c != &' ' {
            break;
        }

        iterator.next();
    }
}

fn expect(iterator: &mut Peekable<Chars>, expected: char) -> Result<(), String> {
    skip_whitespaces(iterator);

    if let Some(c) = iterator.next() {
        if c != expected {
            return Err(format!("Expected '{}' instead received {c}", expected));
        }
    } else {
        return Err(format!("Expected '{}'", expected));
    }

    Ok(())
}

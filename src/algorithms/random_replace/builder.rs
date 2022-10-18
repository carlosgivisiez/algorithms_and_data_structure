use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::ReplaceConfig;

pub struct RandomReplaceBuilder<'a> {
    text: &'a str,
    rng: Pcg64,
    to_be_replaced: Option<char>,
    replacement: Option<char>,
    max: u16,
}

impl<'a> RandomReplaceBuilder<'a> {
    pub fn new(text: &'a str, config: ReplaceConfig) -> Self {
        let rng: Pcg64 = Seeder::from(config.seed).make_rng();

        RandomReplaceBuilder {
            text,
            rng,
            to_be_replaced: None,
            replacement: None,
            max: config.max,
        }
    }

    pub fn random_replace(mut self) -> Self {
        if let None = self.to_be_replaced {
            self.to_be_replaced = Some(self.rng.gen::<char>());
        } else {
            self.rng.gen::<char>();
        }

        self.replacement = Some(self.rng.gen::<char>());

        self
    }
}

impl<'a> Into<String> for RandomReplaceBuilder<'a> {
    fn into(self) -> String {
        let mut replaces_count = 0;

        if let Some(to_be_replaced) = self.to_be_replaced {
            if let Some(replacement) = self.replacement {
                return self
                    .text
                    .chars()
                    .map(|c| {
                        if replaces_count < self.max && c == to_be_replaced {
                            replaces_count += 1;
                            replacement
                        } else {
                            c
                        }
                    })
                    .collect();
            }
        }

        return self.text.into();
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomReplaceBuilder, ReplaceConfig};

    #[test]
    fn simple_replace() {
        let builder = RandomReplaceBuilder::new(
            "\u{e9e2c}teste\u{e9e2c}",
            ReplaceConfig::from_seed("simple_seed".into()),
        )
        .random_replace();

        assert_eq!(Into::<String>::into(builder), "\u{804d7}teste\u{804d7}");
    }

    #[test]
    fn multiple_replace() {
        let builder = RandomReplaceBuilder::new(
            "\u{e9e2c}teste\u{e9e2c}",
            ReplaceConfig::from_seed("simple_seed".into()),
        )
        .random_replace()
        .random_replace();

        assert_eq!(Into::<String>::into(builder), "\u{ee5e7}teste\u{ee5e7}");
    }
}

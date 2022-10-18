use crate::{RandomReplaceBuilder, ReplaceConfig};

pub trait RandomReplace {
    fn random_replace(&self, config: ReplaceConfig) -> String;
}

impl RandomReplace for String {
    fn random_replace(&self, config: ReplaceConfig) -> String {
        RandomReplaceBuilder::new(self, config)
            .random_replace()
            .into()
    }
}

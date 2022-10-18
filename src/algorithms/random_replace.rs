use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
pub mod builder;
pub mod simple;

pub struct ReplaceConfig {
    seed: String,
    max: u16,
}

impl ReplaceConfig {
    pub fn from_seed(seed: String) -> Self {
        Self {
            seed,
            max: u16::MAX,
        }
    }

    pub fn from_max(max: u16) -> Self {
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();

        thread_rng().fill(&mut seed);

        Self {
            seed: String::from_utf8_lossy(&seed).into_owned(),
            max,
        }
    }
}

impl Default for ReplaceConfig {
    fn default() -> Self {
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();

        thread_rng().fill(&mut seed);

        Self {
            seed: String::from_utf8_lossy(&seed).into_owned(),
            max: u16::MAX,
        }
    }
}

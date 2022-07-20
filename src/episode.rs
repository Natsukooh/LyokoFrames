use anyhow::{ensure, Result};
use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};

const EPISODES_AMOUNT: usize = 96;

// represents an episode ID.
// the ID is ensured to be valid when instantiating this type via the new function.
#[derive(Copy, Clone)]
pub struct EpisodeNumber {
    number: usize,
}

impl EpisodeNumber {
    pub fn new(number: usize) -> Result<EpisodeNumber> {
        ensure!(
            number < EPISODES_AMOUNT,
            "Incorrect episode number, expected something between 0 and {}, got {}",
            EPISODES_AMOUNT - 1,
            number
        );
        Ok(EpisodeNumber { number })
    }

    pub fn folder_path(self) -> PathBuf {
        Path::new("episodes").join(self.number.to_string())
    }

    pub fn generate_episodes_order() -> [EpisodeNumber; EPISODES_AMOUNT] {
        let mut episodes = [EpisodeNumber::new(0).unwrap(); 96];
        let mut rng = rand::thread_rng();
        (0..96).for_each(|i| episodes[i] = EpisodeNumber::new(i).unwrap());

        (0..10).for_each(|_| episodes.shuffle(&mut rng));
        episodes
    }
}

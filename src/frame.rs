use crate::episode::EpisodeNumber;
use crate::util::Content;
use anyhow::{ensure, Result as AnyhowResult};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

const FRAMES_PER_EPISODE: usize = 6;

#[derive(Copy, Clone)]
pub struct Frame {
    episode_number: EpisodeNumber,
    frame_number: usize,
}

impl Frame {
    pub fn new(episode_number: EpisodeNumber, frame_number: usize) -> AnyhowResult<Frame> {
        ensure!(
            frame_number < FRAMES_PER_EPISODE,
            "Incorrect frame number, expected something between 0 and {}, got {}",
            FRAMES_PER_EPISODE - 1,
            frame_number
        );
        Ok(Frame {
            episode_number,
            frame_number,
        })
    }

    pub fn frame_path(&self) -> PathBuf {
        let mut temp_path = self
            .episode_number
            .folder_path()
            .join(self.frame_number.to_string());
        let _ = temp_path.set_extension("jpg");
        temp_path
    }
}

impl Content for Frame {
    fn content(&self) -> Result<Vec<u8>, Error> {
        fs::read(self.frame_path())
    }
}

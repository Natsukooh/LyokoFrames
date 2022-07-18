use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fs;
use warp::Filter;

use anyhow::{Context, ensure, Result};
use rand::seq::SliceRandom;

const EPISODES_AMOUNT: usize = 96;
const FRAMES_PER_EPISODE: usize = 6;
const API_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[tokio::main]
async fn main() -> Result<()> {
    let episodes_order = EpisodeNumber::generate_episodes_order();

    let index_page = Arc::new(read_index_page()?);
    let game_page = Arc::new(read_game_page()?);

    let index = warp::get().and(warp::path("index.html")).map(move || {
        let index_page = Arc::clone(&index_page);
        warp::reply::html(index_page.to_string())
    });

    let game = warp::get().and(warp::path("game.html")).map(move || {
        let game_page = Arc::clone(&game_page);
        warp::reply::html(game_page.to_string())
    });

    let status = warp::get().and(warp::path("status")).map(|| "ok");

    let frame = warp::get()
        .and(warp::path!("frame" / usize))
        .map(|frame: usize| "toudou");

    let routes = index.or(game).or(status).or(frame);

    let server = warp::serve(routes);

    server
        .run(SocketAddr::new(IpAddr::V4(API_ADDRESS), 8080))
        .await;

    Ok(())
}

fn read_index_page() -> Result<String> {
    fs::read_to_string("static/index.html").context("Error reading the index.html file")
}

fn read_game_page() -> Result<String> {
    fs::read_to_string("static/game.html").context("Error reading the game.html file")
}

// represents an episode ID.
// the ID is ensured to be valid when instantiating this type via the new function.
#[derive(Copy, Clone)]
struct EpisodeNumber {
    number: usize,
}

impl EpisodeNumber {
    fn new(number: usize) -> Result<EpisodeNumber> {
        ensure!(number < EPISODES_AMOUNT, "Incorrect episode number, expected something between 0 and {}, got {}", EPISODES_AMOUNT, number);
        Ok(EpisodeNumber{number})
    }

    fn folder_path(self) -> PathBuf {
        Path::new("episodes").join(self.number.to_string())
    }

    fn generate_episodes_order() -> [EpisodeNumber; EPISODES_AMOUNT] {
        let mut episodes = [EpisodeNumber::new(0).unwrap(); 96];
        let mut rng = rand::thread_rng();
        (0..96).for_each(|i| episodes[i] = EpisodeNumber::new(i).unwrap());

        (0..10).for_each(|_| episodes.shuffle(&mut rng));
        episodes
    }
}

#[derive(Copy, Clone)]
struct Frame {
    episode_number: EpisodeNumber,
    frame_number: usize
}

impl Frame {
    fn new(episode_number: EpisodeNumber, frame_number: usize) -> Result<Frame>
    {
        ensure!(frame_number < FRAMES_PER_EPISODE, "Incorrect frame number, expected something between 0 and {}, got {}", FRAMES_PER_EPISODE, frame_number);
        Ok(Frame{ episode_number, frame_number })
    }

    fn frame_path(self) -> Result<PathBuf> {
        let mut temp_path = self.episode_number.folder_path().join(self.frame_number.to_string());
        let _ = temp_path.set_extension("jpg");
        Ok(temp_path)
    }
}

#![feature(async_fn_in_trait)]

pub use reqwest;

pub mod lyric;
pub mod song;

pub mod types;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct QQMusicApi {
    base_url: reqwest::Url,
}

impl QQMusicApi {
    pub fn new(base_url: reqwest::Url) -> Self {
        Self { base_url }
    }
}

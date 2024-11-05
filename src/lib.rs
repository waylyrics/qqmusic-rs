pub use http;
pub use serde;
pub use serde_json;
pub use url;

pub mod types;

pub mod lyric;
pub mod search;
pub mod song;
pub mod user;

pub use types::SongId;

mod config;

pub use config::QQMusicApi;

pub type GETResult = http::Result<http::Request<()>>;
pub type POSTResult = http::Result<http::Request<String>>;

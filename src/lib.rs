pub use serde;
pub use serde_json;

pub mod types;

pub use types::SongId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QQMusicApi {
    base_url: url::Url,
}

impl QQMusicApi {
    pub fn new(base_url: url::Url) -> Self {
        Self { base_url }
    }
}

/// 获取歌词
pub mod lyric;
/// 搜索
pub mod search;
/// 歌曲信息
pub mod song;
/// Cookies
pub mod user;

pub use url;

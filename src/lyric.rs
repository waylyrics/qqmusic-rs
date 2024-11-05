use crate::{GETResult, QQMusicApi, SongId};

pub trait QueryLyric {
    fn query_lyric(&self, song: &str) -> GETResult;
}

impl QueryLyric for QQMusicApi {
    fn query_lyric(&self, mid: &str) -> GETResult {
        let mut url = self.base_url.clone();
        url.set_path("/lyric");
        url.set_query(Some(&SongId::Songmid(mid).to_string()));

        http::Request::builder()
            .uri(url.as_str())
            .method("GET")
            .body(())
    }
}

pub use crate::types::lyric::QueryLyricResp;

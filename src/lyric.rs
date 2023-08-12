use url::Url;

use crate::{SongId, QQMusicApi};

pub trait QueryLyric {
    fn query_lyric(&self, song: SongId<'_>) -> Url;
}

impl QueryLyric for QQMusicApi {
    fn query_lyric(&self,  id: SongId<'_>) -> Url {
        let mut url = self.base_url.clone();
        url.set_path("/lyric");
        url.set_query(Some(&id.to_string()));
        url
    }
}

pub use crate::types::lyric::QueryLyricResp;

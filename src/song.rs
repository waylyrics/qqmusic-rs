pub use crate::types::song::SongDetailResp;

use crate::{GETResult, QQMusicApi, SongId};

pub trait SongDetail {
    fn song_detail(&self, id: SongId<'_>) -> GETResult;
}

impl SongDetail for QQMusicApi {
    fn song_detail(&self, id: SongId<'_>) -> GETResult {
        let mut url = self.base_url.clone();
        url.set_path("/song");
        url.set_query(Some(&id.to_string()));

        http::Request::builder()
            .method("GET")
            .uri(url.as_str())
            .body(())
    }
}

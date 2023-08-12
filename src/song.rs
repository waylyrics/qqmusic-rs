use url::Url;
use crate::SongId;

pub use crate::types::song::SongDetailResp;

use crate::QQMusicApi;

pub trait SongDetail {
    fn song_detail(&self, id: SongId<'_>) -> Url;
}

impl SongDetail for QQMusicApi {
    fn song_detail(&self, id: SongId<'_>) -> Url{
        let mut url = self.base_url.clone();
        url.set_path("/song");
        url.set_query(Some(&id.to_string()));
        url
    }
}

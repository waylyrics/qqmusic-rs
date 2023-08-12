use url::Url;

use crate::QQMusicApi;

use search_type::SearchType;

pub trait Search {
    fn search<T: SearchType>(
        &self,
        key: &str,
        page_num: Option<u32>,
        page_size: Option<u32>,
    ) -> Url;
}

impl Search for QQMusicApi {
    fn search<T: SearchType>(
        &self,
        key: &str,
        page_num: Option<u32>,
        page_size: Option<u32>,
    ) -> Url {
        let mut url = self.base_url.clone();

        url.set_path("/search");

        let mut query = format!("key={key}&t={}", T::TYPE_ID);

        if let Some(page_num) = page_num {
            query += "&pageNo=";
            query += &page_num.to_string();
        }
        if let Some(page_size) = page_size {
            query += "&pageSize=";
            query += &page_size.to_string();
        }

        url.set_query(Some(&query));
        url
    }
}

macro_rules! search_type {
    ( $t:ident, $id:expr, $r:ident ) => {
        pub struct $t;
        impl SearchType for $t {
            const TYPE_ID: u32 = $id;
            type Resp = crate::types::search::search_type::$r;
        }
    };
}

mod search_type {
    pub trait SearchType {
        const TYPE_ID: u32;
        type Resp;
    }

    search_type!(Track, 0, TrackSearchResp);
}
// search_type!(TrackList, 2, TrackListSearchResp);
// search_type!(Track, 7, LyricSearchResp);
// search_type!(Track, 8, AlbumSearchResp);
// search_type!(Track, 9, ArtistSearchResp);
// search_type!(Track, 12, MVSearchResp);

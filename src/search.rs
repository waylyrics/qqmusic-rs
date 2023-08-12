use url::Url;

pub trait Search {
    fn search(&self, key: &str, page_num: Option<u32>, page_size: Option<u32>, t: impl SearchType) -> Url;
}

pub trait SearchType {
    const TYPE_ID: u32;
    type Resp;
}

macro_rules! search_type {
    ( $t:ident, $id:expr, $r:ident ) => {
        struct $t;
        impl SearchType for $t {
            const TYPE_ID: u32 = $id;
            type Resp = crate::types::search::search_type::$r;
        }
    };
}

search_type!(Track, 0, TrackSearchResp);
// search_type!(TrackList, 2, TrackListSearchResp);
// search_type!(Track, 7, LyricSearchResp);
// search_type!(Track, 8, AlbumSearchResp);
// search_type!(Track, 9, ArtistSearchResp);
// search_type!(Track, 12, MVSearchResp);

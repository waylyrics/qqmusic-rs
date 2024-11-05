#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QQMusicApi {
    pub(crate) base_url: url::Url,
}

impl QQMusicApi {
    pub fn new(base_url: url::Url) -> Self {
        Self { base_url }
    }
}

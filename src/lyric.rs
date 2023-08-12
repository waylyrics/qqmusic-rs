use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

use crate::{song::SongId, types::lyric::QueryLyricResp, QQMusicApi};

#[async_trait]
pub trait QueryLyric {
    async fn query_lyric(&self, client: &Client, song: SongId<'_>) -> Result<QueryLyricResp>;
}

#[async_trait]
impl QueryLyric for QQMusicApi {
    async fn query_lyric(&self, client: &Client, id: SongId<'_>) -> Result<QueryLyricResp> {
        let id_param: (&str, &str) = id.into();
        let mut url = self.base_url.clone();
        url.set_path("/lyric");
        let resp = client
            .get(url)
            .query(&[id_param])
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&resp)?)
    }
}

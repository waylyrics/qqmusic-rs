use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

use crate::QQMusicApi;
use crate::types::song::SongDetailResp;

pub enum SongId<'a> {
    Songid(&'a str),
    Songmid(&'a str),
}

impl<'a> From<SongId<'a>> for (&'static str, &'a str) {
    fn from(value: SongId<'a>) -> Self {
        match value {
            SongId::Songid(id) => ("songid", id),
            SongId::Songmid(mid) => ("songmid", mid),
        }
    }
}

#[async_trait]
pub trait SongDetail {
    async fn song_detail(
        &self,
        client: &Client,
        id: SongId<'_>,
    ) -> Result<SongDetailResp>;
}

#[async_trait]
impl SongDetail for QQMusicApi {
    async fn song_detail(
        &self,
        client: &Client,
        id: SongId<'_>,
    ) -> Result<SongDetailResp> {
        let id_param: (&str, &str) = id.into();
        let mut url = self.base_url.clone();
        url.set_path("/song");
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

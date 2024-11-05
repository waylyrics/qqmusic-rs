use std::str::FromStr;

use qqmusic_rs::{
    lyric::{QueryLyric, QueryLyricResp},
    song::{SongDetail, SongDetailResp},
    QQMusicApi, SongId,
};
use reqwest::{Client, Url};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36").build()?;
    let api = QQMusicApi::new(Url::from_str("http://127.0.0.1:3300")?);
    let detail: SongDetailResp = serde_json::from_str(
        &client
            .get(api.song_detail(SongId::Songid("114514"))?.uri().to_string())
            .send()
            .await?
            .text()
            .await?,
    )?;
    let mid = &detail.data.track_info.mid;

    let url = api.query_lyric(&mid)?.uri().to_string();
    let resp: QueryLyricResp = serde_json::from_str(&client.get(url).send().await?.text().await?)?;
    println!("{resp:#?}");
    Ok(())
}

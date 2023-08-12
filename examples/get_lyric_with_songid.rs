
use std::str::FromStr;

use qqmusic_rs::{QQMusicApi, lyric::QueryLyric, song::{SongDetail, SongId}};
use reqwest::{Client, Url};
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let client = Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36").build()?;
    let api = QQMusicApi::new(Url::from_str("http://127.0.0.1:3300")?);
    let detail = api.song_detail(&client, SongId::Songid("114514")).await?;
    let mid = &detail.data.track_info.mid;
    let resp = api.query_lyric(&client, SongId::Songmid(&mid)).await?;
    println!("{resp:#?}");
    Ok(())
}
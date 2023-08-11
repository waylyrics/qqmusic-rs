use qqmusic_rs::lyric::query_lyric;
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let client = Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36").build()?;
    let resp = query_lyric(&client, Err("001afu2a1qjiik".into())).await?;
    println!("{resp:#?}");
    Ok(())
}
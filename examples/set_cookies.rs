use std::str::FromStr;

use qqmusic_rs::{types::user::SetCookieResp, user::SetCookie, QQMusicApi};
use reqwest::{Client, Request, Url};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = QQMusicApi::new(Url::from_str("http://127.0.0.1:3300")?);
    let cookies = "";
    let request = api.set_cookie(cookies)?;

    let client = Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36").build()?;
    let reqwest_req = Request::try_from(request)?;

    let resp: SetCookieResp =
        serde_json::from_str(&client.execute(reqwest_req).await?.text().await?)?;
    println!("{resp:#?}");
    Ok(())
}

// const result = await request({
//   url: '',
//   data: {
//     songmid,
//     pcachetime: new Date().getTime(),
//     g_tk: 5381,
//     loginUin: 0,
//     hostUin: 0,
//     inCharset: 'utf8',
//     outCharset: 'utf-8',
//     notice: 0,
//     platform: 'yqq',
//     needNewCode: 0,
//   },
//   headers: {
//     Referer: 'https://y.qq.com',
//   }
// });

use std::time::UNIX_EPOCH;

use reqwest::Client;

pub async fn query_lyric(client: &Client, song: Result<u32, String>) -> anyhow::Result<String> {
    let pcachetime = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time backwards")
        .as_secs()
        .to_string();

    let mut form = vec![
        ("needNewCode", "0"),
        ("platform", "yqq"),
        ("notice", "0"),
        ("outCharset", "utf-8"),
        ("inCharset", "utf8"),
        ("hostUin", "0"),
        ("loginUin", "0"),
        ("g_tk", "5381"),
        ("pcachetime", &pcachetime),
    ];

    let is_songid = song.is_ok();
    let id = match song {
        Ok(id) => id.to_string(),
        Err(mid) => mid,
    };
    if is_songid {
        form.push(("songid", &id));
    } else {
        form.push(("songmid", &id));
    }

    Ok(client
        .get("http://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg")
        .form(&form)
        .header("Referer", "https://y.qq.com")
        .send()
        .await?
        .text()
        .await?)
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackSearchResp {
    pub result: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub list: Vec<List>,
    #[serde(rename = "pageNo")]
    pub page_no: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub total: i64,
    pub key: String,
    pub t: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub singer: Vec<Singer>,
    pub name: String,
    pub songid: i64,
    pub songmid: String,
    pub songname: String,
    pub albumid: i64,
    pub albummid: String,
    pub albumname: String,
    pub interval: i64,
    #[serde(rename = "strMediaMid")]
    pub str_media_mid: String,
    pub size128: i64,
    pub size320: i64,
    pub sizeape: i64,
    pub sizeflac: i64,
    pub pay: Pay,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Singer {
    pub id: i64,
    pub mid: String,
    pub name: String,
    pub pmid: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub uin: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pay {
    pub pay_down: i64,
    pub pay_month: i64,
    pub pay_play: i64,
    pub pay_status: i64,
    pub price_album: i64,
    pub price_track: i64,
    pub time_free: i64,
}

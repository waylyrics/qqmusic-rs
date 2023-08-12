use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SongDetailResp {
    pub result: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub info: Info,
    pub extras: Extras,
    pub track_info: TrackInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub genre: Genre,
    pub lan: Lan,
    pub pub_time: PubTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<GenreInfo>,
    pub pos: i64,
    pub more: i64,
    pub selected: String,
    pub use_platform: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenreInfo {
    pub id: i64,
    pub value: String,
    pub mid: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub show_type: i64,
    pub is_parent: i64,
    pub picurl: String,
    pub read_cnt: i64,
    pub author: String,
    pub jumpurl: String,
    pub ori_picurl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lan {
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<LanInfo>,
    pub pos: i64,
    pub more: i64,
    pub selected: String,
    pub use_platform: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanInfo {
    pub id: i64,
    pub value: String,
    pub mid: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub show_type: i64,
    pub is_parent: i64,
    pub picurl: String,
    pub read_cnt: i64,
    pub author: String,
    pub jumpurl: String,
    pub ori_picurl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubTime {
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<PubTimeInfo>,
    pub pos: i64,
    pub more: i64,
    pub selected: String,
    pub use_platform: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubTimeInfo {
    pub id: i64,
    pub value: String,
    pub mid: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub show_type: i64,
    pub is_parent: i64,
    pub picurl: String,
    pub read_cnt: i64,
    pub author: String,
    pub jumpurl: String,
    pub ori_picurl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extras {
    pub name: String,
    pub transname: String,
    pub subtitle: String,
    pub from: String,
    pub wikiurl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub mid: String,
    pub name: String,
    pub title: String,
    pub subtitle: String,
    pub singer: Vec<Singer>,
    pub album: Album,
    pub mv: Mv,
    pub interval: i64,
    pub isonly: i64,
    pub language: i64,
    pub genre: i64,
    pub index_cd: i64,
    pub index_album: i64,
    pub time_public: String,
    pub status: i64,
    pub fnote: i64,
    pub file: File,
    pub pay: Pay,
    pub action: Action,
    pub ksong: Ksong,
    pub volume: Volume,
    pub label: String,
    pub url: String,
    pub bpm: i64,
    pub version: i64,
    pub trace: String,
    pub data_type: i64,
    pub modify_stamp: i64,
    pub pingpong: String,
    pub ppurl: String,
    pub tid: i64,
    pub ov: i64,
    pub sa: i64,
    pub es: String,
    pub vs: Vec<String>,
    pub vi: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Singer {
    pub id: i64,
    pub mid: String,
    pub name: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub uin: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub mid: String,
    pub name: String,
    pub title: String,
    pub subtitle: String,
    pub time_public: String,
    pub pmid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mv {
    pub id: i64,
    pub vid: String,
    pub name: String,
    pub title: String,
    pub vt: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub media_mid: String,
    pub size_24aac: i64,
    pub size_48aac: i64,
    pub size_96aac: i64,
    pub size_192ogg: i64,
    pub size_192aac: i64,
    pub size_128mp3: i64,
    pub size_320mp3: i64,
    pub size_ape: i64,
    pub size_flac: i64,
    pub size_dts: i64,
    pub size_try: i64,
    pub try_begin: i64,
    pub try_end: i64,
    pub url: String,
    pub size_hires: i64,
    pub hires_sample: i64,
    pub hires_bitdepth: i64,
    pub b_30s: i64,
    pub e_30s: i64,
    pub size_96ogg: i64,
    // pub size_360ra: Vec<Value>,
    pub size_dolby: i64,
    pub size_new: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pay {
    pub pay_month: i64,
    pub price_track: i64,
    pub price_album: i64,
    pub pay_play: i64,
    pub pay_down: i64,
    pub pay_status: i64,
    pub time_free: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub switch: i64,
    pub msgid: i64,
    pub alert: i64,
    pub icons: i64,
    pub msgshare: i64,
    pub msgfav: i64,
    pub msgdown: i64,
    pub msgpay: i64,
    pub switch2: i64,
    pub icon2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ksong {
    pub id: i64,
    pub mid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub gain: f64,
    pub peak: f64,
    pub lra: f64,
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookieResp {
    pub result: i64,
    pub data: String,
}

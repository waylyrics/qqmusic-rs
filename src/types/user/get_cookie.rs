use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookieResp {
    pub result: i64,
    pub message: String,
}

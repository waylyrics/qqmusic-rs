use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryLyricResp {
    pub result: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub retcode: i64,
    /// code should be 0 on succ request
    pub code: i64,
    pub subcode: i64,
    /// origin lyric, might be empty
    pub lyric: String,
    /// translated lyric, might be empty
    pub trans: String,
}

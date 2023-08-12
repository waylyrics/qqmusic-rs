use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewCookieResp {
    pub result: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "psrf_access_token_expiresAt")]
    pub psrf_access_token_expires_at: String,
    pub psrf_musickey_createtime: String,
    pub psrf_qqrefresh_token: String,
    pub psrf_qqaccess_token: String,
    pub wxrefresh_token: String,
    pub psrf_qqunionid: String,
    #[serde(rename = "_qpsvr_localtk")]
    pub qpsvr_localtk: String,
    pub ptui_loginuin: String,
    pub psrf_qqopenid: String,
    pub fqm_sessionid: String,
    #[serde(rename = "tmeLoginType")]
    pub tme_login_type: String,
    pub qqmusic_key: String,
    pub login_type: String,
    pub fqm_pvqid: String,
    pub wxunionid: String,
    pub pgv_pvid: String,
    pub ts_refer: String,
    pub wxopenid: String,
    pub pgv_info: String,
    pub qm_keyst: String,
    pub eas_sid: String,
    pub pac_uid: String,
    pub ts_last: String,
    pub ts_uid: String,
    pub ptcz: String,
    pub euin: String,
    pub iip: String,
    pub uin: String,
    #[serde(rename = "RK")]
    pub rk: String,
}

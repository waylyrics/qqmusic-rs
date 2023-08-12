use serde_json::{json, Value};
use url::Url;

use crate::QQMusicApi;

/// set cookie to session
pub trait GetCookie {
    /// id: QQ number or WeChat UIN
    fn get_cookie(&self, id: &str) -> Url;
}

/// set cookie to server with POST
///
/// Content-type: application/json
///
/// { "data": "xxx=xxx; xx=xxxx; ..."}
pub trait SetCookie {
    fn set_cookie(&self, cookie: &str) -> (Url, Value);
}

/// view current cookie set in session
pub trait ViewCookie {
    fn cookie(&self) -> Url;
}

impl GetCookie for QQMusicApi {
    fn get_cookie(&self, id: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path("/user/getCookie");
        url.set_query(Some(&format!("id={id}")));
        url
    }
}

impl ViewCookie for QQMusicApi {
    fn cookie(&self) -> Url {
        let mut url = self.base_url.clone();
        url.set_path("/user/cookie");
        url
    }
}

impl SetCookie for QQMusicApi {
    fn set_cookie(&self, cookie: &str) -> (Url, Value) {
        let mut url = self.base_url.clone();
        url.set_path("/setCookie");
        let payload = json!({"data": cookie});
        (url, payload)
    }
}

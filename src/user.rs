use serde_json::json;

use crate::{GETResult, POSTResult, QQMusicApi};

/// set cookie to session
pub trait GetCookie {
    /// id: QQ number or WeChat UIN
    fn get_cookie(&self, id: &str) -> GETResult;
}

/// set cookie to server with POST
pub trait SetCookie {
    fn set_cookie(&self, cookie: &str) -> POSTResult;
}

/// view current cookie set in session
pub trait ViewCookie {
    fn cookie(&self) -> GETResult;
}

impl GetCookie for QQMusicApi {
    fn get_cookie(&self, id: &str) -> GETResult {
        let mut url = self.base_url.clone();
        url.set_path("/user/getCookie");
        url.set_query(Some(&format!("id={id}")));

        http::Request::builder()
            .method("GET")
            .uri(url.as_str())
            .body(())
    }
}

impl ViewCookie for QQMusicApi {
    fn cookie(&self) -> GETResult {
        let mut url = self.base_url.clone();
        url.set_path("/user/cookie");

        http::Request::builder()
            .method("GET")
            .uri(url.as_str())
            .body(())
    }
}

impl SetCookie for QQMusicApi {
    fn set_cookie(&self, cookie: &str) -> POSTResult {
        let mut url = self.base_url.clone();
        url.set_path("/user/setCookie");
        let payload = json!({"data": cookie});

        http::Request::builder()
            .method("POST")
            .uri(url.as_str())
            .header("Content-Type", "application/json")
            .body(payload.to_string())
    }
}

use rand::distributions::{Alphanumeric, Distribution};
use rand::rngs::OsRng;
use serde::Deserialize;
use url::Url;

pub static AUTH_URL: &str = "https://open.weixin.qq.com/connect/oauth2/authorize";
pub static TOKEN_URL: &str = "https://api.weixin.qq.com/sns/oauth2/access_token";
pub static SNSAPI_BASE: &str = "snsapi_base";
pub static STATE_LENGTH: usize = 64;

#[derive(Deserialize)]
pub struct WechatLogin {
    appid: String,
    secret: String,
    pub redirect_uri: Url,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CodeResponse {
    Success {
        access_token: String,
        expires_in: i64,
        refresh_token: String,
        openid: String,
        scope: String,
    },
    Failure {
        errcode: i64,
        errmsg: String,
    },
}

impl WechatLogin {
    pub fn auth_url(&self) -> (String, Url) {
        let state = gen_state();
        let mut url = Url::parse(AUTH_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("appid", &self.appid)
            .append_pair("redirect_uri", self.redirect_uri.as_str())
            .append_pair("response_type", "code")
            .append_pair("scope", SNSAPI_BASE)
            .append_pair("state", &state);
        url.set_fragment(Some("wechat_redirect"));
        (state, url)
    }

    pub async fn request_id(&self, code: &str) -> anyhow::Result<CodeResponse> {
        let url = self.open_id_url(code);
        let client = reqwest::Client::new();
        let rsp = client.get(url).send().await?.json::<CodeResponse>().await?;
        Ok(rsp)
    }

    fn open_id_url(&self, code: &str) -> Url {
        let mut url = Url::parse(TOKEN_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("appid", &self.appid)
            .append_pair("secret", &self.secret)
            .append_pair("code", code)
            .append_pair("grant_type", "authorization_code");
        url
    }
}

#[inline]
fn gen_state() -> String {
    String::from_iter(
        Alphanumeric
            .sample_iter(OsRng::default())
            .take(STATE_LENGTH)
            .map(|u| u as char),
    )
}

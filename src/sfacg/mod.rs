mod novel;
mod user;
mod volumes;

use reqwest::{IntoUrl, RequestBuilder};
use serde::Deserialize;

use crate::utils;
pub use novel::*;
pub use user::*;
pub use volumes::*;

pub struct SfClient {
  http: reqwest::Client,
}

impl SfClient {
  pub fn new() -> Self {
    Self {
      http: reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap(),
    }
  }

  fn get(&self, url: impl IntoUrl) -> RequestBuilder {
    self
      .http
      .get(url)
      .basic_auth("androiduser", Some("1a#$51-yt69;*Acv@qxq"))
      .header("sfsecurity", utils::sf_security())
      .header(
        "user-agent",
        format!(
          "boluobao/5.0.74(android;31)/H5/{}/H5",
          utils::device_token()
        ),
      )
  }

  fn post(&self, url: impl IntoUrl) -> RequestBuilder {
    self
      .http
      .post(url)
      .basic_auth("androiduser", Some("1a#$51-yt69;*Acv@qxq"))
      .header("sfsecurity", utils::sf_security())
      .header(
        "user-agent",
        format!(
          "boluobao/5.0.74(android;31)/H5/{}/H5",
          utils::device_token()
        ),
      )
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SfRespStatus {
  // http_code: i32,
  // error_code: i32,
  // msg_type: i32,
  msg: Option<String>,
}

#[derive(Deserialize)]
struct SfResp<T> {
  status: SfRespStatus,
  data: Option<T>,
}

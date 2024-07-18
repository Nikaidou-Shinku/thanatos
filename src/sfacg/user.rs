use eyre::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{SfClient, SfResp};

pub struct UserInfo {
  pub id: i32,
  pub username: String,
  pub nickname: String,
  /// may be empty
  pub email: String,
  pub avatar: String,
  pub is_author: bool,
  // TODO: use date type instead
  pub register_date: String,
}

impl SfClient {
  pub async fn login(&self, username: impl AsRef<str>, password: impl AsRef<str>) -> Result<()> {
    #[derive(Serialize)]
    struct LoginPayload<'a> {
      username: &'a str,
      password: &'a str,
    }

    let payload = LoginPayload {
      username: username.as_ref(),
      password: password.as_ref(),
    };

    let resp = self
      .post("https://api.sfacg.com/sessions")
      .json(&payload)
      .send()
      .await?;

    if !resp.status().is_success() {
      let res: SfResp<()> = resp.json().await?;
      bail!("Login failed: {:?}", res.status.msg);
    }

    Ok(())
  }

  pub async fn user_info(&self) -> Result<UserInfo> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SfRespUserInfo {
      user_name: String,
      // country_code: i32,
      nick_name: String,
      email: String,
      account_id: i32,
      // role_name: String,
      // fire_coin: i32,
      avatar: String,
      is_author: bool,
      // phone_num: String,
      register_date: String,
    }

    impl From<SfRespUserInfo> for UserInfo {
      fn from(value: SfRespUserInfo) -> Self {
        Self {
          id: value.account_id,
          username: value.user_name,
          nickname: value.nick_name,
          email: value.email,
          avatar: value.avatar,
          is_author: value.is_author,
          register_date: value.register_date,
        }
      }
    }

    let res: SfResp<SfRespUserInfo> = self
      .get("https://api.sfacg.com/user")
      .send()
      .await?
      .json()
      .await?;

    let Some(data) = res.data else {
      bail!("Get user info failed: {:?}", res.status.msg);
    };

    Ok(data.into())
  }
}

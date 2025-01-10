use eyre::{Result, bail};
use serde::{Deserialize, Serialize};

use super::{SfClient, SfResp};

#[derive(Serialize)]
pub struct NovelInfoAuthor {
  pub id: i32,
  pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelInfo {
  pub id: i32,
  pub title: String,
  pub author: NovelInfoAuthor,
  pub cover: String,
  pub char_count: i32,
  pub is_finish: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfRespNovelInfo {
  author_id: i32,
  // last_update_time: String,
  // mark_count: i32,
  novel_cover: String,
  // bg_banner: String,
  novel_id: i32,
  novel_name: String,
  // point: f64,
  is_finish: bool,
  author_name: String,
  char_count: i32,
  // view_times: i32,
  // type_id: i32,
  // allow_down: bool,
  // add_time: String,
  // is_sensitive: bool,
  // sign_status: String,
  // category_id: i32,
}

impl From<SfRespNovelInfo> for NovelInfo {
  fn from(value: SfRespNovelInfo) -> Self {
    Self {
      id: value.novel_id,
      title: value.novel_name,
      author: NovelInfoAuthor {
        id: value.author_id,
        name: value.author_name,
      },
      cover: value.novel_cover,
      char_count: value.char_count,
      is_finish: value.is_finish,
    }
  }
}

impl SfClient {
  #[tracing::instrument(skip(self))]
  pub async fn novel_info(&self, novel_id: i32) -> Result<NovelInfo> {
    tracing::info!("Requesting...");

    let res: SfResp<SfRespNovelInfo> = self
      .get(format!("https://api.sfacg.com/novels/{novel_id}"))
      .send()
      .await?
      .json()
      .await?;

    let Some(data) = res.data else {
      tracing::warn!(message = res.status.msg, "Failed");
      bail!("Get novel info failed: {:?}", res.status.msg);
    };

    tracing::info!("Ok");

    Ok(data.into())
  }
}

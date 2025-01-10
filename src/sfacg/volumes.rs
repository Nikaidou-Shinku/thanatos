use eyre::{Result, bail};
use serde::{Deserialize, Serialize};

use super::{SfClient, SfResp};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterInfo {
  pub id: i32,
  pub title: String,
  pub char_count: i32,
  pub create_time: String,
  pub update_time: Option<String>,
}

pub struct VolumeInfo {
  pub id: i32,
  pub title: String,
  pub list: Vec<ChapterInfo>,
}

pub struct VolumesInfo {
  pub id: i32,
  pub list: Vec<VolumeInfo>,
}

impl SfClient {
  #[tracing::instrument(skip(self))]
  pub async fn volumes_info(&self, novel_id: i32) -> Result<VolumesInfo> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SfRespChapterInfo {
      chap_id: i32,
      // novel_id: i32,
      // volume_id: i32,
      // need_fire_money: i32,
      // origin_need_fire_money: i32,
      // chapter_origin_fire_money: i32,
      char_count: i32,
      // row_num: i32,
      // chap_order: i32,
      title: String,
      // content: Option<()>,
      // sno: f64,
      // is_vip: bool,
      #[serde(rename = "AddTime")]
      add_time: String,
      update_time: Option<String>,
      // can_unlock_with_ad: bool,
      // ntitle: String,
      // is_rubbish: bool,
      // audit_status: i32,
    }

    impl From<SfRespChapterInfo> for ChapterInfo {
      fn from(value: SfRespChapterInfo) -> Self {
        Self {
          id: value.chap_id,
          title: value.title,
          char_count: value.char_count,
          create_time: value.add_time,
          update_time: value.update_time,
        }
      }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SfRespVolumeInfo {
      volume_id: i32,
      title: String,
      // sno: f64,
      chapter_list: Vec<SfRespChapterInfo>,
    }

    impl From<SfRespVolumeInfo> for VolumeInfo {
      fn from(value: SfRespVolumeInfo) -> Self {
        Self {
          id: value.volume_id,
          title: value.title,
          list: value.chapter_list.into_iter().map(Into::into).collect(),
        }
      }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SfRespVolumesInfo {
      novel_id: i32,
      // last_update_time: String,
      volume_list: Vec<SfRespVolumeInfo>,
    }

    impl From<SfRespVolumesInfo> for VolumesInfo {
      fn from(value: SfRespVolumesInfo) -> Self {
        Self {
          id: value.novel_id,
          list: value.volume_list.into_iter().map(Into::into).collect(),
        }
      }
    }

    tracing::info!("Requesting...");

    let res: SfResp<SfRespVolumesInfo> = self
      .get(format!("https://api.sfacg.com/novels/{novel_id}/dirs"))
      .send()
      .await?
      .json()
      .await?;

    let Some(data) = res.data else {
      tracing::warn!(message = res.status.msg, "Failed");
      bail!("Get volumes info failed: {:?}", res.status.msg);
    };

    tracing::info!("Ok");

    Ok(data.into())
  }
}

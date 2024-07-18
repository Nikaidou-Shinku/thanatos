use eyre::{bail, Result};
use serde::Deserialize;

use super::{novel::SfRespNovelInfo, NovelInfo, SfClient, SfResp};

impl SfClient {
  pub async fn search(&self, keyword: impl AsRef<str>) -> Result<Vec<NovelInfo>> {
    #[derive(Deserialize)]
    struct SfRespSearchNovel {
      #[serde(flatten)]
      base: SfRespNovelInfo,
      // weight: i32,
      // #[serde(rename = "Highlight")]
      // highlight: Vec<String>,
    }

    #[derive(Deserialize)]
    struct SfRespSearch {
      novels: Vec<SfRespSearchNovel>,
      // comics: Vec<()>,
      // albums: Vec<()>,
      // booklist: Vec<()>,
    }

    let res: SfResp<SfRespSearch> = self
      .get("https://api.sfacg.com/search/novels/result/new")
      .query(&[("q", keyword.as_ref()), ("size", "12")])
      .send()
      .await?
      .json()
      .await?;

    let Some(data) = res.data else {
      bail!("Search failed: {:?}", res.status.msg);
    };

    Ok(data.novels.into_iter().map(|n| n.base.into()).collect())
  }
}

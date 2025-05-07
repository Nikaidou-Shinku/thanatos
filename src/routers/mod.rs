mod chapter;
mod novel;
mod search;
mod version;

use novel_api::NovelInfo;
use serde::Serialize;

pub use chapter::*;
pub use novel::*;
pub use search::*;
pub use version::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelInfoResp {
  id: u32,
  title: String,
  author: String,
  cover: String,
  char_count: u32,
  is_finish: bool,
}

impl From<NovelInfo> for NovelInfoResp {
  fn from(value: NovelInfo) -> Self {
    Self {
      id: value.id,
      title: value.name,
      author: value.author_name,
      // TODO: fallback when there's no cover
      cover: value.cover_url.unwrap().into(),
      char_count: value.word_count.unwrap_or_default(),
      is_finish: value.is_finished.unwrap_or_default(),
    }
  }
}

mod chapter;
mod novel;
mod search;
mod version;

use std::sync::Arc;

use axum::{Router, routing::get};
use novel_api::{Client, NovelInfo};
use serde::Serialize;

use crate::{Clients, Dispatch};
pub use version::version;

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

pub fn routes<C: Dispatch + Client + 'static>() -> Router<Arc<Clients>> {
  Router::new()
    .route("/novels/{id}", get(novel::novel::<C>))
    .route("/novels/{id}/chapters", get(chapter::chapters::<C>))
    .route("/search", get(search::search::<C>))
}

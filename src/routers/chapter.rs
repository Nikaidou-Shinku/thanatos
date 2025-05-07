use std::sync::Arc;

use axum::{
  Json,
  extract::{Path, State},
};
use novel_api::{ChapterInfo, Client};
use serde::Serialize;

use crate::{Clients, Dispatch};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterInfoResp {
  id: u32,
  title: String,
  char_count: u32,
  create_time: String,
  update_time: Option<String>,
}

impl From<ChapterInfo> for ChapterInfoResp {
  fn from(value: ChapterInfo) -> Self {
    Self {
      id: value.id,
      title: value.title,
      char_count: value.word_count.unwrap_or_default(),
      // TODO: fallback when `create_time` is None
      create_time: value.create_time.unwrap_or_default().to_string(),
      update_time: value.update_time.map(|x| x.to_string()),
    }
  }
}

#[tracing::instrument(skip(clients), fields(platform))]
pub async fn chapters<C: Dispatch + Client>(
  Path(novel_id): Path<u32>,
  State(clients): State<Arc<Clients>>,
) -> Json<Vec<ChapterInfoResp>> {
  let client = <C as Dispatch>::dispatch(clients.as_ref());

  tracing::Span::current().record("platform", client.platform());

  tracing::info!("GET /novels/{novel_id}/chapters");

  let volumes = match client.volume_infos(novel_id).await {
    Ok(Some(res)) => res,
    Ok(None) => {
      tracing::info!("Not found");
      return Json(Vec::new());
    }
    Err(error) => {
      tracing::warn!(%error, "Failed");
      return Json(Vec::new());
    }
  };

  let res: Vec<_> = volumes
    .into_iter()
    .flat_map(|v| v.chapter_infos)
    .map(Into::into)
    .collect();

  tracing::info!(count = res.len(), "Ok");

  Json(res)
}

use std::sync::Arc;

use axum::{
  Json,
  extract::{Path, State},
};
use novel_api::{Client, SfacgClient};

use super::NovelInfoResp;

#[tracing::instrument(skip(client))]
pub async fn novel(
  Path(novel_id): Path<u32>,
  State(client): State<Arc<SfacgClient>>,
) -> Json<Option<NovelInfoResp>> {
  tracing::info!("GET /novels/{novel_id}");

  let novel = match client.novel_info(novel_id).await {
    Ok(Some(res)) => res,
    Ok(None) => {
      tracing::info!("Not found");
      return Json(None);
    }
    Err(error) => {
      tracing::warn!(%error, "Failed");
      return Json(None);
    }
  };

  tracing::info!(name = novel.name, "Ok");

  Json(Some(novel.into()))
}

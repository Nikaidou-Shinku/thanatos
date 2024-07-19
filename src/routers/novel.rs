use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use thanatos::sfacg::{NovelInfo, SfClient};

#[tracing::instrument(skip(client))]
pub async fn novel(
  Path(novel_id): Path<i32>,
  State(client): State<Arc<SfClient>>,
) -> Json<Option<NovelInfo>> {
  tracing::info!("GET /novels/{novel_id}");

  let novel = match client.novel_info(novel_id).await {
    Ok(res) => res,
    Err(error) => {
      tracing::warn!(%error, "Failed");
      return Json(None);
    }
  };

  tracing::info!(name = novel.title, "Ok");

  Json(Some(novel))
}

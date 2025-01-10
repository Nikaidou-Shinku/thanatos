use std::sync::Arc;

use axum::{
  Json,
  extract::{Path, State},
};
use thanatos::sfacg::{ChapterInfo, SfClient};

#[tracing::instrument(skip(client))]
pub async fn chapters(
  Path(novel_id): Path<i32>,
  State(client): State<Arc<SfClient>>,
) -> Json<Vec<ChapterInfo>> {
  tracing::info!("GET /novels/{novel_id}/chapters");

  let volumes = match client.volumes_info(novel_id).await {
    Ok(res) => res,
    Err(error) => {
      tracing::warn!(%error, "Failed");
      return Json(Vec::new());
    }
  };

  let res: Vec<_> = volumes.list.into_iter().flat_map(|v| v.list).collect();

  tracing::info!(count = res.len(), "Ok");

  Json(res)
}

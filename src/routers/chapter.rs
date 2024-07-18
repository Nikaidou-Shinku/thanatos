use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use thanatos::sfacg::{ChapterInfo, SfClient};

pub async fn chapters(
  Path(novel_id): Path<i32>,
  State(client): State<Arc<SfClient>>,
) -> Json<Vec<ChapterInfo>> {
  let volumes = client.volumes_info(novel_id).await.unwrap();
  let res = volumes.list.into_iter().map(|v| v.list).flatten().collect();

  Json(res)
}

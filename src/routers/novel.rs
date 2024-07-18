use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use thanatos::sfacg::{NovelInfo, SfClient};

pub async fn novel(
  Path(novel_id): Path<i32>,
  State(client): State<Arc<SfClient>>,
) -> Json<NovelInfo> {
  let novel = client.novel_info(novel_id).await.unwrap();

  Json(novel)
}

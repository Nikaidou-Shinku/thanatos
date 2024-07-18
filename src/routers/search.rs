use std::sync::Arc;

use axum::{
  extract::{Query, State},
  Json,
};
use serde::Deserialize;
use thanatos::sfacg::{NovelInfo, SfClient};

#[derive(Deserialize)]
pub struct SearchParams {
  keyword: String,
}

pub async fn search(
  Query(params): Query<SearchParams>,
  State(client): State<Arc<SfClient>>,
) -> Json<Vec<NovelInfo>> {
  let novels = client.search(params.keyword).await.unwrap();

  Json(novels)
}

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

#[tracing::instrument(skip_all, fields(keyword = params.keyword))]
pub async fn search(
  Query(params): Query<SearchParams>,
  State(client): State<Arc<SfClient>>,
) -> Json<Vec<NovelInfo>> {
  tracing::info!("GET /search");

  let novels = match client.search(params.keyword).await {
    Ok(res) => res,
    Err(error) => {
      tracing::warn!(%error, "Failed");
      return Json(Vec::new());
    }
  };

  tracing::info!(
    count = novels.len(),
    first = novels.first().map(|n| &n.title),
    "Ok",
  );

  Json(novels)
}

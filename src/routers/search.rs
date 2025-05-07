use std::sync::Arc;

use axum::{
  Json,
  extract::{Path, Query, State},
};
use novel_api::{Client, Options, SfacgClient};
use serde::Deserialize;

use super::NovelInfoResp;

#[derive(Deserialize)]
pub struct SearchParams {
  keyword: String,
}

#[tracing::instrument(skip_all, fields(keyword = params.keyword))]
pub async fn search(
  Query(params): Query<SearchParams>,
  State(client): State<Arc<SfacgClient>>,
) -> Json<Vec<NovelInfoResp>> {
  tracing::info!("GET /search");

  let novels = match client
    .search_infos(
      &Options {
        keyword: Some(params.keyword),
        ..Default::default()
      },
      0,
      12,
    )
    .await
  {
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

  tracing::info!(count = novels.len(), "Search Ok");

  // TODO: upstream do not return detailed info
  let handles: Vec<_> = novels
    .into_iter()
    .map(|id| {
      (
        id,
        tokio::spawn(super::novel(Path(id), State(client.clone()))),
      )
    })
    .collect();

  let mut novels = Vec::with_capacity(handles.len());

  for (id, handle) in handles {
    let Ok(Json(novel)) = handle.await else {
      tracing::warn!(id, "Failed to get info");
      continue;
    };

    match novel {
      Some(novel) => novels.push(novel),
      None => {
        tracing::warn!(id, "Not found");
      }
    }
  }

  tracing::info!(first = novels.first().map(|n| &n.title), "Get info Ok");

  Json(novels)
}

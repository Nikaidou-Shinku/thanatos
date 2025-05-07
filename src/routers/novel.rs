use std::sync::Arc;

use axum::{
  Json,
  extract::{Path, State},
};
use novel_api::Client;

use super::NovelInfoResp;
use crate::{Clients, Dispatch};

#[tracing::instrument(skip(clients), fields(platform))]
pub async fn novel<C: Dispatch + Client>(
  Path(novel_id): Path<u32>,
  State(clients): State<Arc<Clients>>,
) -> Json<Option<NovelInfoResp>> {
  let client = <C as Dispatch>::dispatch(clients.as_ref());

  tracing::Span::current().record("platform", client.platform());

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

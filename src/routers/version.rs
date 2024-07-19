use axum::Json;
use serde::Serialize;

use crate::{BUILD_TIME, GIT_HASH, VERSION};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResp {
  version: &'static str,
  git_hash: &'static str,
  build_time: u128,
}

#[tracing::instrument]
pub async fn version() -> Json<VersionResp> {
  tracing::info!("GET /version");

  Json(VersionResp {
    version: VERSION,
    git_hash: GIT_HASH,
    build_time: BUILD_TIME.parse().unwrap(),
  })
}

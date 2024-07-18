use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResp {
  version: &'static str,
  git_hash: &'static str,
  build_time: u128,
}

pub async fn version() -> Json<VersionResp> {
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  const GIT_HASH: &str = env!("GIT_HASH");
  const BUILD_TIME: &str = env!("BUILD_TIME");

  Json(VersionResp {
    version: VERSION,
    git_hash: GIT_HASH,
    build_time: BUILD_TIME.parse().unwrap(),
  })
}

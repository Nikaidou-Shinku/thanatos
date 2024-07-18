mod routers;

use std::sync::Arc;

use axum::{routing::get, Router};
use eyre::Result;
use thanatos::sfacg::SfClient;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  let client = SfClient::new();
  let app = Router::new()
    .route("/novels/:id", get(routers::novel))
    .route("/novels/:id/chapters", get(routers::chapters))
    .with_state(Arc::new(client));
  let listener = TcpListener::bind("0.0.0.0:9961").await?;

  axum::serve(listener, app).await?;
  Ok(())
}

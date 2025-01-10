mod constants;
mod routers;

use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use eyre::Result;
use thanatos::sfacg::SfClient;
use tokio::net::TcpListener;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use constants::*;

#[tokio::main]
async fn main() -> Result<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .with_target(false)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("failed to set default subscriber");

  tracing::info!(VERSION, GIT_HASH, BUILD_TIME, "Starting Thanatos...");

  let client = SfClient::default();
  let app = Router::new()
    .route("/version", get(routers::version))
    .route("/novels/{id}", get(routers::novel))
    .route("/novels/{id}/chapters", get(routers::chapters))
    .route("/search", get(routers::search))
    .with_state(Arc::new(client));
  let addr: SocketAddr = "0.0.0.0:9961".parse().unwrap();
  let listener = TcpListener::bind(addr).await?;

  tracing::info!(%addr, "Listening...");

  axum::serve(listener, app).await?;
  Ok(())
}

mod constants;
mod routers;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Router, routing::get};
use dotenvy_macro::dotenv;
use novel_api::{CiweimaoClient, Client, SfacgClient};
use tokio::net::TcpListener;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use constants::*;

trait Dispatch {
  fn dispatch(clients: &Clients) -> &Self;
}

struct Clients {
  sfacg: SfacgClient,
  ciweimao: CiweimaoClient,
}

impl Clients {
  async fn new() -> Result<Self> {
    let ciweimao = CiweimaoClient::new().await?;

    ciweimao
      .log_in(
        dotenv!("CIWEIMAO_USERNAME").to_owned(),
        Some(dotenv!("CIWEIMAO_PASSWORD").to_owned()),
      )
      .await?;

    Ok(Self {
      sfacg: SfacgClient::new().await?,
      ciweimao,
    })
  }
}

impl Dispatch for SfacgClient {
  fn dispatch(clients: &Clients) -> &Self {
    &clients.sfacg
  }
}

impl Dispatch for CiweimaoClient {
  fn dispatch(clients: &Clients) -> &Self {
    &clients.ciweimao
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .with_target(false)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("failed to set default subscriber");

  tracing::info!(VERSION, GIT_HASH, BUILD_TIME, "Starting Thanatos...");

  let clients = Clients::new().await.expect("Failed to build the clients");

  let app = Router::new()
    .nest("/sfacg", routers::routes::<SfacgClient>())
    .nest("/ciweimao", routers::routes::<CiweimaoClient>())
    .route("/version", get(routers::version))
    .with_state(Arc::new(clients));

  let addr: SocketAddr = "0.0.0.0:9961".parse().unwrap();
  let listener = TcpListener::bind(addr).await?;

  tracing::info!(%addr, "Listening...");

  axum::serve(listener, app).await?;

  Ok(())
}

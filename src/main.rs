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
  fn platform(&self) -> &'static str;
}

struct Clients {
  sfacg: SfacgClient,
  ciweimao: CiweimaoClient,
}

impl Clients {
  async fn new() -> Result<Self> {
    let ciweimao = CiweimaoClient::new().await?;

    if !ciweimao.logged_in().await? {
      ciweimao
        .log_in(
          dotenv!("CIWEIMAO_USERNAME").to_owned(),
          Some(dotenv!("CIWEIMAO_PASSWORD").to_owned()),
        )
        .await?;
    }

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

  fn platform(&self) -> &'static str {
    "SFACG"
  }
}

impl Dispatch for CiweimaoClient {
  fn dispatch(clients: &Clients) -> &Self {
    &clients.ciweimao
  }

  fn platform(&self) -> &'static str {
    "CIWEIMAO"
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

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;

  Ok(())
}

async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending();

  tokio::select! {
    _ = ctrl_c => { }
    _ = terminate => { }
  }
}

mod constants;
mod routes;
mod state;
mod types;
mod tg;

use dotenv::dotenv;
use tg::connect_client;
use std::sync::Arc;

use axum::{Extension, Router};
use routes::{fallback, routes};
use state::AppState;
use tokio::{net::TcpListener, sync::RwLock};

use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let listener = TcpListener::bind(format!("{}:{}", "localhost", "8001"))
        .await
        .unwrap();

    let client = connect_client().await?;

    let mut state = AppState::default();

    state.set_tg_client(client);

    let shared_state = Arc::new(RwLock::new(state));

    let router = Router::new()
        .nest("/api/v1", routes())
        .layer(Extension(shared_state))
        .fallback(fallback);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

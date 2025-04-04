mod constants;
mod routes;
mod state;
mod types;

use dotenv::dotenv;
use std::sync::Arc;

use axum::{Extension, Router};
use routes::{fallback, routes};
use state::AppState;
use tokio::fs;
use tokio::{net::TcpListener, sync::RwLock};

use grammers_client::{Client, Config, SignInError, Update};
use grammers_session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let listener = TcpListener::bind(format!("{}:{}", "localhost", "8001"))
        .await
        .unwrap();

    let session_file = "session.session";
    let session = if let Ok(data) = fs::read(session_file).await {
        Session::load(&data)?
    } else {
        Session::new()
    };

    let client = Client::connect(Config {
        session,
        api_id: 0,
        api_hash: "0".to_string(),
        params: Default::default(),
    })
    .await?;

    println!("Server starting on port 8001");

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

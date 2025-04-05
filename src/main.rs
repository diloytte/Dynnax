mod constants;
mod pf;
mod routes;
mod state;
mod tg;
mod types;

use dotenv::dotenv;
use std::{env, sync::Arc};
use tg::{connect_client, listen_for_updates};

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
    
    state.set_tg_client(client.clone());
    
    let shared_state = Arc::new(RwLock::new(state));
    
    let pf_api_key: String = env::var("PUMPFUN_PORTAL_API_KEY")?.parse()?;
    tokio::spawn(listen_for_updates(client, shared_state.clone(),pf_api_key));

    let router = Router::new()
        .nest("/api/v1", routes())
        .layer(Extension(shared_state))
        .fallback(fallback);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

mod constants;
mod models;
mod pf;
mod routes;
mod state;
mod tg;

use dotenv::dotenv;
use tg::{client::connect_client, dialog::get_dialogs::get_dialogs, snipe::snipe::snipe};
use std::{env, sync::Arc};

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

    let dialogs = get_dialogs(&client).await?;

    let mut state = AppState::default();

    state.set_tg_client(client.clone());

    // IMPORTANT NOTICE:
    //
    // As of now, we're using DashMap for concurrent access to the `snipe_targets` in `AppState`
    // which provides internal mutability and thread-safety without requiring an `RwLock` around the
    // entire state. This is why we don't need to wrap the entire `AppState` in `RwLock`. The `DashMap`
    // takes care of synchronization internally, allowing us to modify entries within it safely across
    // multiple threads. This leads to simpler and more performant code, as we avoid locking the entire
    // state for every read and write operation.
    //
    // When will `RwLock` be needed?
    //
    // `RwLock` will only be necessary if we add other types to `AppState` that are not inherently
    // thread-safe (like `HashMap` or `bool`) and need to ensure synchronized access across the entire
    // state object. In those cases, we would either:
    // 1. Wrap the fields individually in `Mutex` or `RwLock` for synchronization.
    // 2. Wrap the entire `AppState` in a `RwLock` (as we had before) to ensure consistency across
    //    all fields.
    //
    // But for now, DashMap is handling all the heavy lifting for concurrent access to `snipe_targets`,
    // so there's no need for an `RwLock` around the entire `AppState` object.
    let shared_state = Arc::new(state);

    let pf_api_key: String = env::var("PUMPFUN_PORTAL_API_KEY")?.parse()?;

    tokio::spawn(snipe(
        client.clone(),
        shared_state.clone(),
        pf_api_key.clone(),
    ));

    // tokio::spawn(snipe_x(client,shared_state.clone(),pf_api_key));

    let router = Router::new()
        .nest("/api/v1", routes())
        .layer(Extension(shared_state))
        .fallback(fallback);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

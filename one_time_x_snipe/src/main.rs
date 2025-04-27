mod routes;
mod state;

use std::sync::Arc;

use axum::Router;
use dotenv::dotenv;
use routes::routes;
use shared::db::connect::connect;
use state::State;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port = 8002;
    let listener = TcpListener::bind(format!("{}:{}", "localhost", port))
        .await
        .unwrap();

    let db_url = env::var("DATABASE_URL")?;
    let db = connect(db_url).await.unwrap();

    let state = State { db };

    let shared_state = Arc::new(state);

    let router = Router::new().nest("/api/v1", routes());

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

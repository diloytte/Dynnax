mod routes;

use axum::Router;
use routes::{fallback, routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(format!("{}:{}", "localhost", "8001"))
        .await
        .unwrap();

    println!("Server starting on port 8001");

    let router = Router::new().nest("/api/v1", routes()).fallback(fallback);

    axum::serve(listener, router).await.unwrap();
}

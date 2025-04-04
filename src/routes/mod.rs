mod dbg;

use axum::{body::Body, http::{Request, StatusCode}, response::IntoResponse, Router};

pub fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    let mut router = Router::new();
    router = router.merge(dbg::routes());
    router
}

pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (StatusCode::NOT_FOUND, format!("That endpoint '{}' is not in our API.", path))
}
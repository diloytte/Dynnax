mod dbg;
mod snipe;
mod tg;
mod pf;
mod snipe_x;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
};

pub fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    let router = Router::new();
    router
        .merge(dbg::routes())
        .merge(tg::routes())
        .merge(snipe::routes())
        .merge(pf::routes())
        .merge(snipe_x::routes())
}

pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("That endpoint '{}' is not in our API.", path),
    )
}

mod dbg;
mod pf;
mod snipe;
mod snipe_internal;
mod snipe_x;
mod snipe_x_internal;
mod tg;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    middleware,
    response::IntoResponse,
};

use crate::middlewares;

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
        .layer(middleware::from_fn(middlewares::auth_middleware))
}

pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("That endpoint '{}' is not in our API.", path),
    )
}

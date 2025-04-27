use std::sync::Arc;

use axum::Extension;

use crate::state::AppState;

pub type AppStateExtension = Extension<Arc<AppState>>;

use core::fmt;
use std::{io, path::Display, sync::Arc};

use axum::{extract::MatchedPath, Extension};
use serde::Serialize;
use sqlx::ValueRef;

use crate::state::AppState;

pub type AppStateExtension = Extension<Arc<AppState>>;
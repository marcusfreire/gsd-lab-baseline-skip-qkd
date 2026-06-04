use axum::{Router, routing::get};
use std::sync::Arc;

use crate::AppState;

mod etsi014;

pub fn build_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/v1/keys", etsi014::build_router())
        .with_state(app_state)
}

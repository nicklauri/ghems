use anyhow::Result;
use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::{io, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

use crate::{api, clientapp, config::CONFIG};

pub async fn main_router() -> Result<Router> {
    Ok(Router::new()
        .nest("/api", api::router().await?)
        .fallback(get_service(clientapp::serve_dir_service()).handle_error(handle_error)))
}

pub async fn handle_error(err: io::Error) -> impl IntoResponse {
    warn!("clientapp::serve_dir_service: {}", err);

    let error_message = if CONFIG.is_production() {
        String::new()
    } else {
        err.to_string()
    };

    (StatusCode::INTERNAL_SERVER_ERROR, error_message)
}

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use axum_macros::FromRequest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::{
    config::CONFIG,
    extractors::dev::DevOnly,
    models::user::User,
    utils::{self, api_success, password},
};

use super::ApiResult;

pub fn router() -> Router {
    Router::new()
        .route("/", get(dev_help))
        .route("/hash", get(hash))
}

pub async fn dev_help(_: DevOnly) -> impl IntoResponse {
    r#"
    hash: POST /api/dev/hash
        - body: { input: string }
    "#
}

#[derive(Debug, Deserialize)]
pub struct HashRequest {
    input: String,
}

pub async fn hash(_: DevOnly, param: Query<HashRequest>) -> ApiResult<String> {
    info!("request: hash \"{}\"", param.input);

    let (hashed, salt) = password::hash(&param.input).await?;

    api_success(format!(
        "
            input : {}
            hashed: {}
            salt  : {}
            hashed.len={}, salt.len={}
        ",
        param.input,
        hashed,
        salt,
        hashed.len(),
        salt.len()
    ))
}

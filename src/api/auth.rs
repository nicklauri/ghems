use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

use super::ApiResult;

pub fn router() -> Router {
    Router::new().route("/endpoint", post(oauth_endpoint))
}

#[derive(Debug, Deserialize)]
pub struct OAuthEndpointRequest {
    code: Option<String>,
    error: Option<String>,
}

pub async fn oauth_endpoint() -> ApiResult<()> {
    Ok(())
}

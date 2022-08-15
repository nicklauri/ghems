use axum::{
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::{
    api::{ApiContext, ApiResult},
    db::{self, Db},
    error::Error,
    models::User,
    utils::{self, password},
    GhemResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    token: String,
    schema: &'static str,
    user: User,
}

// #[axum_macros::debug_handler]
pub async fn login(
    ctx: Extension<ApiContext>,
    req: Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let user = db::user_account::get_user_by_username(&ctx.db, &req.username)
        .await?
        .ok_or(Error::Unauthorized)?;

    // Verify password and return unauthorized reponse so we don't have to manually check.
    user.verify_password(&req.password).await?;

    Ok(Json(LoginResponse {
        token: utils::generate_jwt_token(user.id, user.email.clone(), &ctx),
        schema: &ctx.config.jwt_schema,
        user,
    }))
}

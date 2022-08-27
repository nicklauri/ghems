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
    models::{role::UserRole, User},
    utils::{self, password},
    GResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
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

    let roles = db::user_role::get_user_roles_by_user_id(&ctx.db, user.id).await?;

    let access_token = user.generate_access_token(&roles, &ctx).await;
    let refresh_token = user.generate_refresh_token().await;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        schema: &ctx.config.jwt_schema,
        user,
    }))
}

use axum::{
    extract::Query,
    handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::{
    db,
    error::Error,
    extractors::{
        auth::AuthUser,
        roles::{Admin, Member},
    },
    models::user::{UpdateUser, User},
    utils::{self, api_failure, api_response, api_success},
};

use super::{ApiContext, ApiResponse, ApiResult};

pub mod login;

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_user).put(update_user).delete(delete_user))
        .route("/login", post(login::login))
        .route("/admin", get(admin_only))
        .route("/member", get(member))
        .route("/info", get(get_info))
}

pub async fn get_info(auth: AuthUser, ctx: Extension<ApiContext>) -> ApiResult<Option<User>> {
    let user = db::user_account::get_user_info_by_user_id(ctx.db(), auth.user_id).await?;

    api_response(user.is_some(), user)
}

pub async fn create_user(auth: AuthUser) -> impl IntoResponse {
    (StatusCode::OK, "Here is some secrets\n")
}

pub async fn admin_only(_: Admin) -> impl IntoResponse {
    (StatusCode::OK, "Admin's secret\n")
}

pub async fn member(_: Member) -> impl IntoResponse {
    (StatusCode::OK, "Member's secret\n")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    email: String,
    first_name: String,
    last_name: String,
    bio: String,
    password: String,
}
pub async fn update_user(
    auth: AuthUser,
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<()> {
    let (password_hash, salt) = utils::password::hash(&req.password).await?;

    let user_info = UpdateUser {
        id: auth.user_id,
        email: req.email,
        bio: req.bio,
        first_name: req.first_name,
        last_name: req.last_name,
        password_hash,
        salt,
    };

    db::user_account::update_user(ctx.db(), user_info).await?;

    api_success(())
}

pub async fn delete_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

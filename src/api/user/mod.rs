use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::{
    extractors::{
        auth::AuthUser,
        roles::{Admin, Member},
    },
    models::user::User,
    utils::{self},
};

pub mod login;

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_user).put(update_user).delete(delete_user))
        .route("/login", post(login::login))
        .route("/admin", get(admin_only))
        .route("/member", get(member))
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

pub async fn update_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn delete_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

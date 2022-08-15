use argon2::password_hash::SaltString;
use axum::{http::StatusCode, response::IntoResponse, response::Response};
use uuid::Uuid;

use crate::{api::ApiContext, extractors::auth::AuthUser, GhemResult};

pub mod crypto;
pub mod dev;
pub mod env;
pub mod password;

#[inline]
pub fn ok<T>(res: T) -> GhemResult<Response<T>>
where
    T: IntoResponse,
{
    get_response(StatusCode::OK, res)
}

#[inline]
pub fn unauthorized<T>(res: T) -> GhemResult<Response<T>>
where
    T: IntoResponse,
{
    get_response(StatusCode::UNAUTHORIZED, res)
}

#[inline]
pub fn get_response<T>(status: StatusCode, res: T) -> GhemResult<Response<T>>
where
    T: IntoResponse,
{
    let response = Response::builder().status(status).body(res)?;

    Ok(response)
}

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

pub fn generate_jwt_token(user_id: Uuid, email: String, ctx: &ApiContext) -> String {
    let auth_user = AuthUser { user_id, email };

    auth_user.to_jwt(ctx)
}

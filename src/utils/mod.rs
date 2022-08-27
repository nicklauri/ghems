use argon2::password_hash::SaltString;
use axum::{http::StatusCode, response::IntoResponse, response::Response};
use uuid::Uuid;

use crate::{api::ApiContext, config::Config, extractors::auth::AuthUser, GResult};

pub mod crypto;
pub mod env;
pub mod error;
pub mod password;

#[inline]
pub fn ok<T>(res: T) -> GResult<Response<T>>
where
    T: IntoResponse,
{
    get_response(StatusCode::OK, res)
}

#[inline]
pub fn unauthorized<T>(res: T) -> GResult<Response<T>>
where
    T: IntoResponse,
{
    get_response(StatusCode::UNAUTHORIZED, res)
}

#[inline]
pub fn get_response<T>(status: StatusCode, res: T) -> GResult<Response<T>>
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

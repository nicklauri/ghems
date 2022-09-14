use argon2::password_hash::SaltString;
use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    api::{ApiContext, ApiResponse, ApiResult},
    config::Config,
    error::Error,
    extractors::auth::AuthUser,
    GResult,
};

use self::cow::ToCow;

pub mod cow;
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

#[inline]
pub fn api_response<T: Serialize>(is_success: bool, data: T) -> ApiResult<T> {
    Ok(Json(ApiResponse::new(is_success, None, data)))
}

#[inline]
pub fn api_response_msg<T: Serialize>(is_success: bool, msg: &str, data: T) -> ApiResult<T> {
    Ok(Json(ApiResponse::new(
        is_success,
        Some(msg.to_string()),
        data,
    )))
}

#[inline]
pub fn api_success<T: Serialize>(data: T) -> ApiResult<T> {
    api_response(true, data)
}

#[inline]
pub fn api_failure<T: Serialize>(msg: &'static str) -> ApiResult<T> {
    Err(Error::ApiError(Some(msg.to_cow())))
}

#[inline]
pub fn api_failure_msg<T: Serialize>(msg: &str) -> ApiResult<T> {
    Err(Error::ApiError(Some(msg.to_string().to_cow())))
}

use std::borrow::Cow;
use std::error::Error as StdError;

use axum::http::{self, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;

use sqlx::error::DatabaseError;
use tracing::warn;
use validator::ValidationErrors;

use crate::api::ApiResponse;
use crate::config::CONFIG;

pub type GResult<T> = Result<T, Error>;

/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("an internal server error occurred")]
    ResponseBuilder(#[from] http::Error),

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{}", .0.as_ref().map(crate::utils::cow::borrow_str).unwrap_or("API return error"))]
    ApiError(Option<Cow<'static, str>>),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            errors: Option<&'a ValidationErrors>,

            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<String>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        // Normally you wouldn't just print this, but it's useful for debugging without
        // using a logging framework.
        warn!("api error: {:?}", self);

        let details = if CONFIG.is_development() {
            Some(format!("{:#?}", self))
        } else {
            None
        };

        (
            self.status_code(),
            Json(ApiResponse {
                is_success: false,
                message: Some(self.to_string()),
                data: ErrorResponse { errors, details },
            }),
        )
            .into_response()
    }
}

impl Error {
    #[inline]
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Anyhow(_) | ResponseBuilder(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) => StatusCode::BAD_REQUEST,
            Unauthorized => StatusCode::UNAUTHORIZED,
            Forbidden => StatusCode::FORBIDDEN,
            NotFound => StatusCode::NOT_FOUND,
            ApiError(_) => StatusCode::OK,
        }
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&ctxt.db)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
///
/// Something like this would ideally live in a `sqlx-axum` crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `Error` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}

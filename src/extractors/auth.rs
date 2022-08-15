use std::borrow::{Borrow, Cow};
/// JsonWebToken authentication
use std::ops::Add;

use axum::body::Body;
use axum::extract::{Extension, FromRequest, RequestParts};

use async_trait::async_trait;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use jsonwebtoken::{Algorithm, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::error::Error;
use crate::GhemResult;

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Bearer <token>` header. Authorization schema can be set from .env file.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
///
/// If the `Authorization` header is absent then this will be `Self(None)`, otherwise it will
/// validate the token.
///
/// This is in contrast to directly using `Option<AuthUser>`, which will be `None` if there
/// is *any* error in deserializing, which isn't exactly what we want.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(Serialize, Deserialize)]
struct AuthUserClaims<'a> {
    user_id: Uuid,
    sub: Cow<'a, str>,
    exp: i64,
}

impl AuthUser {
    pub fn to_jwt(&self, ctx: &ApiContext) -> String {
        let claims = AuthUserClaims {
            user_id: self.user_id,
            sub: Cow::from(&self.email),
            exp: OffsetDateTime::now_utc()
                .add(ctx.config.jwt_max_session_length)
                .unix_timestamp(),
        };

        jsonwebtoken::encode(&Header::new(Algorithm::HS384), &claims, ctx.encoding_key())
            .expect("HS384 should not be panicked")
    }

    /// Attempt to parse `Self` from an `Authorization` header.
    fn from_authorization(ctx: &ApiContext, auth_header: &HeaderValue) -> GhemResult<Self> {
        let auth_header = auth_header.to_str().map_err(|_| {
            debug!("authorization header is not valid UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(&ctx.config.jwt_schema) {
            debug!(
                "authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
        }

        let token = auth_header[ctx.config.jwt_schema.len()..].trim();

        let jwt = jsonwebtoken::decode::<AuthUserClaims<'static>>(
            token,
            &ctx.decoding_key,
            &Validation::new(Algorithm::HS384),
        )
        .map_err(|e| {
            debug!(
                "failed to parse `authorization` header {:?}: {}",
                auth_header, e
            );
            Error::Unauthorized
        })?;

        if jwt.claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            debug!("token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            user_id: jwt.claims.user_id,
            email: jwt.claims.sub.to_string(),
        })
    }
}

impl MaybeAuthUser {
    /// If this is `Self(Some(AuthUser))`, return `AuthUser::user_id`
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|auth_user| auth_user.user_id)
    }

    pub fn is_authenticated(&self) -> bool {
        self.0.is_some()
    }
}

// tower-http has a `RequireAuthorizationLayer` but it's useless for practical applications,
// as it only supports matching Basic or Bearer auth with credentials you provide it.
//
// There's the `::custom()` constructor to provide your own validator but it basically
// requires parsing the `Authorization` header by-hand anyway so you really don't get anything
// out of it that you couldn't write your own middleware for, except with a bunch of extra
// boilerplate.
#[async_trait]
impl<B: Send> FromRequest<B> for AuthUser {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request(req)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for MaybeAuthUser {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request(req)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        Ok(Self(
            // Get the value of the `Authorization` header, if it was sent at all.
            req.headers()
                .get(AUTHORIZATION)
                .and_then(|auth_header| Some(AuthUser::from_authorization(&ctx, auth_header)))
                .transpose()?,
        ))
    }
}

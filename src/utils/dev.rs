use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{api::ApiContext, error::Error};

/// Add this as a parameter to a handler function to require development environment.
#[derive(Debug, Serialize, Deserialize)]
pub struct DevOnly;

#[async_trait]
impl<B: Send> FromRequest<B> for DevOnly {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let _ = req
            .extensions()
            .get::<ApiContext>()
            .ok_or_else(|| {
                error!("BUG: ApiContext was not added as an extension");
                Error::Forbidden
            })
            .map(Some)?
            .filter(|ctx| ctx.config.allow_dev_routes())
            .ok_or(Error::Forbidden)?;

        Ok(Self)
    }
}

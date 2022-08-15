use std::{ops::Deref, sync::Arc};

use axum::{response::Response, Extension, Json, Router};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::{
    config::{Config, CONFIG},
    db::Db,
    GhemResult,
};

pub mod auth;
pub mod dev;
pub mod user;

pub type ApiResult<T> = GhemResult<T>;

#[derive(Clone)]
pub struct ApiContext {
    inner: Arc<ApiContextInner>,
}

impl ApiContext {
    pub fn new(db: Db) -> Self {
        Self {
            inner: Arc::new(ApiContextInner::new(db)),
        }
    }
}

impl Deref for ApiContext {
    type Target = ApiContextInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone)]
pub struct ApiContextInner {
    pub config: &'static Config,
    pub db: Db,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl ApiContextInner {
    pub fn new(db: Db) -> Self {
        let config = &*CONFIG;
        let encoding_key = match EncodingKey::from_base64_secret(&CONFIG.jwt_hmac_key) {
            Ok(key) => key,
            Err(err) => panic!("Create encoding key from HMAC failed: {:?}", err),
        };

        let decoding_key = match DecodingKey::from_base64_secret(&CONFIG.jwt_hmac_key) {
            Ok(key) => key,
            Err(err) => panic!("Create decoding key from HMAC failed: {:?}", err),
        };

        Self {
            config,
            db,
            encoding_key,
            decoding_key,
        }
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }

    pub fn decoding_key(&self) -> &DecodingKey {
        &self.decoding_key
    }
}

pub async fn router() -> anyhow::Result<Router> {
    info!(
        database_connections = CONFIG.database_connections,
        "connecting to db"
    );

    let pgconn = PgPoolOptions::new()
        .max_connections(CONFIG.database_connections)
        .connect(&CONFIG.database_url)
        .await?;

    let context = ApiContext::new(pgconn);

    let router = Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .nest("/dev", dev::router())
        .layer(Extension(context));

    Ok(router)
}

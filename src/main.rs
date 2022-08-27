#![allow(warnings)]
#![deny(unused_must_use)]

pub mod api;
pub mod clientapp;
pub mod config;
pub mod db;
pub mod error;
pub mod extractors;
pub mod models;
pub mod router;
pub mod utils;

use std::sync::Arc;

use anyhow::Result;
use axum::{Router, Server};
use config::{Environment, CONFIG};
use mimalloc::MiMalloc;
use tracing::info;

pub use error::GResult;

#[global_allocator]
static GLOBAL_ALLOCATOR: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    utils::env::load_environment_variables();
    utils::env::init_logging();

    info!("run mode: {}", CONFIG.environment);

    serve_http(router::main_router().await?, CONFIG.port).await
}

async fn serve_http(app: Router, port: u16) -> Result<()> {
    let addr = ([0, 0, 0, 0], port).into();

    info!("serve: http://localhost:{port}");

    Ok(Server::bind(&addr).serve(app.into_make_service()).await?)
}

use tracing_subscriber::prelude::*;

use crate::config::Environment;

pub fn load_environment_variables() {
    // If the environment is development, load .env.development to substitute variables.
    let mode = env!("ENVIRONMENT");

    let is_development = Environment::from_str(mode)
        .map(|mode| mode.is_development())
        .unwrap_or(false);

    if is_development {
        dotenvy::from_filename(".env.development")
            .map_err(|e| eprintln!("dotenv.development: {}", e))
            .ok();
    }

    // Load default .env
    dotenvy::dotenv()
        .map_err(|e| eprintln!("dotenv: {}", e))
        .ok();
}

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "ghem=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

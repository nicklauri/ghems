use std::{
    env,
    fmt::{self, Display},
};

use anyhow::{bail, Result};
use clap::Parser;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::parse());

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(env)]
    pub database_url: String,

    #[clap(env)]
    pub database_connections: u32,

    #[clap(env, value_parser = parse_list)]
    pub oauth2_issuers: Vec<String>,

    #[clap(env, value_parser = parse_list)]
    pub oauth2_audiences: Vec<String>,

    #[clap(env)]
    pub jwt_hmac_key: String,

    #[clap(env, value_parser = parse_schema, default_value = "Bearer")]
    pub jwt_schema: String,

    #[clap(env, value_parser = parse_time, default_value = "1209600")]
    pub jwt_max_session_length: time::Duration,

    #[clap(env, value_parser = Environment::from_str, default_value = "prod")]
    pub environment: Environment,

    #[clap(env, default_value = "80")]
    pub port: u16,
}

impl Config {
    #[inline]
    /// There maybe a new environment that support dev routes.
    pub const fn allow_dev_routes(&self) -> bool {
        self.is_development()
    }

    #[inline]
    /// Check if the environment is development.
    pub const fn is_development(&self) -> bool {
        self.environment.is_development()
    }

    #[inline]
    /// Check if the environment is production.
    pub const fn is_production(&self) -> bool {
        self.environment.is_production()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn from_str(env: &str) -> Result<Self> {
        Ok(match env {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            _ => bail!("invalid environment: {env}"),
        })
    }

    #[inline]
    pub const fn to_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }

    #[inline]
    pub const fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }

    #[inline]
    pub const fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

impl Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_str())
    }
}

fn parse_time(str: &str) -> Result<time::Duration> {
    let length = str.parse::<i64>()?;

    Ok(time::Duration::seconds(length))
}

fn parse_schema(schema: &str) -> Result<String> {
    let mut schema = String::from(schema);
    schema.push_str(" ");
    Ok(schema)
}

fn parse_list(input: &str) -> Result<Vec<String>> {
    let results = input
        .split([',', ':', ';'])
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .collect();

    Ok(results)
}

use std::str::FromStr;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::error::Error;

use super::auth::AuthUser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Admin,
    Member,
}

impl Role {
    #[inline]
    pub fn to_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Member => "member",
        }
    }
}

impl FromStr for Role {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "member" => Ok(Role::Member),
            _ => Err(Error::Anyhow(anyhow!("invalid role: {}", s))),
        }
    }
}

impl ToString for Role {
    #[inline]
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Debug)]
pub struct Admin(pub AuthUser);

#[derive(Debug)]
pub struct Member(pub AuthUser);

pub trait UserRole {
    fn to_role(&self) -> Role;
}

impl UserRole for Admin {
    fn to_role(&self) -> Role {
        Role::Admin
    }
}

impl UserRole for Member {
    fn to_role(&self) -> Role {
        Role::Member
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for Admin {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let claims = AuthUser::from_request(req).await?;

        if !claims.has_role(Role::Admin) {
            let uri = req
                .uri()
                .path_and_query()
                .map(|s| s.as_str())
                .unwrap_or("/");
            warn!(roles = ?claims.roles, uri = uri, "access admin role");
            return Err(Error::Forbidden);
        }

        Ok(Self(claims))
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for Member {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let claims = AuthUser::from_request(req).await?;

        if !claims.has_role(Role::Member) {
            let uri = req
                .uri()
                .path_and_query()
                .map(|s| s.as_str())
                .unwrap_or("/");
            warn!(roles = ?claims.roles, uri = uri, "access member role");
            return Err(Error::Forbidden);
        }

        Ok(Self(claims))
    }
}

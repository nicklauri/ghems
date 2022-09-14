use time::Time;
use tracing::{error, warn};

use crate::{
    api::ApiContext,
    error::Error,
    extractors::{auth::AuthUser, roles::Role},
    utils, GResult,
};

use super::db_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub picture: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: String,

    #[serde(skip_serializing)]
    pub salt: String,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub const TABLE_NAME: &'static str = "user_account";
    pub const CONSTRAINT_PRIMARY_KEY: &'static str = "user_account_pkey";
    pub const CONSTRAINT_UNIQUE_USERNAME_KEY: &'static str = "user_account_username_key";
    pub const CONSTRAINT_UNIQUE_EMAIL_KEY: &'static str = "user_account_email_key";

    pub fn full_name(&self) -> String {
        [&*self.first_name, &*self.last_name].join(" ")
    }

    #[must_use]
    pub async fn verify_password(&self, password: &str) -> GResult<()> {
        match utils::password::verify(password, &self.password_hash, &self.salt).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(Error::Unauthorized),
            Err(err) => {
                error!("verify_password: {err:?}");
                Err(Error::Unauthorized)
            }
        }
    }

    pub async fn generate_access_token(&self, roles: &[Role], ctx: &ApiContext) -> String {
        let auth_user = AuthUser {
            user_id: self.id.clone(),
            email: self.email.clone(),
            roles: roles.to_vec(),
        };

        auth_user.to_jwt(ctx)
    }

    pub async fn generate_refresh_token(&self) -> String {
        error!("unimplemented generate_refresh_token");
        String::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoShort {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAndRoles {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    #[serde(skip_serializing)]
    pub salt: String,
}

#[allow(dead_code)]
async fn __validate(db: &Db) {
    sqlx::query_as!(
        User,
        r#"
            select *
            from user_account
        "#
    )
    .fetch_one(db)
    .await
    .unwrap();
}

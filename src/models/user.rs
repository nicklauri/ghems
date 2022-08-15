use time::Time;
use tracing::{error, warn};

use crate::{error::Error, utils, GhemResult};

use super::db_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub async fn verify_password(&self, password: &str) -> GhemResult<()> {
        match utils::password::verify(password, &self.password_hash, &self.salt).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(Error::Unauthorized),
            Err(err) => {
                error!("verify_password: {err:?}");
                Err(Error::Unauthorized)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoShort {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
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

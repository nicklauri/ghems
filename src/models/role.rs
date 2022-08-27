use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{self, Db},
    GResult,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRole {
    pub id: Uuid,
    pub role_name: String,
    pub role_ident: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRoles {
    pub id: Uuid,
    pub role_id: Uuid,
    pub user_id: Uuid,
}

#[allow(dead_code)]
async fn __validate(db: &Db) {
    sqlx::query_as!(
        UserRole,
        r#"
            select *
            from user_role
        "#
    )
    .fetch_one(db)
    .await
    .unwrap();

    sqlx::query_as!(
        UserRoles,
        r#"
            select *
            from users_roles
        "#
    )
    .fetch_one(db)
    .await
    .unwrap();
}

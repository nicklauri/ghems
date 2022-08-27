use std::str::FromStr;

use uuid::Uuid;

use crate::{extractors::roles::Role, models::role::UserRole, utils::error::LogError, GResult};

use super::Db;

pub async fn get_roles_by_user_id(db: &Db, user_id: Uuid) -> GResult<Vec<UserRole>> {
    let result = sqlx::query_as!(
        UserRole,
        r#"
            select ur.*
            from user_role ur
            join users_roles urs on urs.role_id = ur.id
            where
                urs.user_id = $1::uuid
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(result)
}

pub async fn get_user_roles_by_user_id(db: &Db, user_id: Uuid) -> GResult<Vec<Role>> {
    let roles = get_roles_by_user_id(db, user_id).await?;

    let roles = roles
        .into_iter()
        .filter_map(|role| Role::from_str(&role.role_ident).log_warn().ok())
        .collect();

    Ok(roles)
}

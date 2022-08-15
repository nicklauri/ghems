use super::Db;

use crate::{models::user::User, GhemResult};

pub async fn get_user_by_username(db: &Db, username: &str) -> GhemResult<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "
            select
                *
            from
                user_account
            where
                username = $1
            limit 1
        ",
        username
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}

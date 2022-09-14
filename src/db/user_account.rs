use uuid::Uuid;

use super::Db;

use crate::{
    error::{Error, ResultExt},
    models::user::{UpdateUser, User, UserInfoShort},
    utils::cow::ToCow,
    GResult,
};

pub async fn get_user_by_username(db: &Db, username: &str) -> GResult<Option<User>> {
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

pub async fn get_user_info_by_user_id(db: &Db, user_id: Uuid) -> GResult<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "
            select
                *
            from
                user_account
            where
                id = $1
            limit 1
        ",
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn update_user(db: &Db, user_info: UpdateUser) -> GResult<()> {
    let user = sqlx::query_scalar!(
        "
            update user_account
            set
                email = $2,
                bio = $3,
                first_name = $4,
                last_name = $5,
                password_hash = $6,
                salt = $7
            where
                id = $1
        ",
        user_info.id,
        user_info.email,
        user_info.bio,
        user_info.first_name,
        user_info.last_name,
        user_info.password_hash,
        user_info.salt,
    )
    .fetch_one(db)
    .await
    .on_constraint(User::CONSTRAINT_UNIQUE_EMAIL_KEY, |_| {
        Error::ApiError(Some("Email is duplicated".to_cow()))
    });

    Ok(())
}

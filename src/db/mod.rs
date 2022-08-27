pub mod user_account;
pub mod user_role;

use sqlx::PgPool;

pub type Db = PgPool;

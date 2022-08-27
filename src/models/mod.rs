pub mod role;
pub mod user;

pub use user::User;

mod db_types {
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::types::{time::OffsetDateTime, uuid::Uuid};

    pub trait Serde<'de>: Deserialize<'de> + Serialize {}
    impl<'de, T: Serialize + Deserialize<'de>> Serde<'de> for T {}

    // for validation:
    pub use crate::db::Db;
}

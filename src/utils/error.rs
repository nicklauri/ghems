use std::borrow::Cow;

use tracing::warn;

use crate::{error::Error, GResult};

pub trait LogError
where
    Self: Sized,
{
    fn get_message(&self) -> Option<Cow<str>>;

    fn log_warn(self) -> Self {
        if let Some(msg) = self.get_message() {
            warn!("{}", msg);
        }

        self
    }

    fn log_warn_context(self, ctx: &str) -> Self {
        if let Some(msg) = self.get_message() {
            warn!("{}: {}", ctx, msg);
        }

        self
    }
}

impl<T> LogError for GResult<T> {
    fn get_message(&self) -> Option<Cow<str>> {
        match self {
            Ok(res) => None,
            Err(err) => Some(Cow::Owned(err.to_string())),
        }
    }
}

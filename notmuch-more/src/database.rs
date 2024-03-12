use crate::NotmuchMoreError;
use std::path::Path;

pub struct Database {
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn open_ro(&self) -> Result<notmuch::Database, NotmuchMoreError> {
        Ok(notmuch::Database::open_with_config(
            Some(&self.path),
            notmuch::DatabaseMode::ReadOnly,
            None::<&Path>,
            None,
        )?)
    }

    pub fn open_rw(&self) -> Result<notmuch::Database, NotmuchMoreError> {
        Ok(notmuch::Database::open_with_config(
            Some(&self.path),
            notmuch::DatabaseMode::ReadWrite,
            None::<&Path>,
            None,
        )?)
    }
}

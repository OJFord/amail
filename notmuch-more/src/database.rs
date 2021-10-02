use crate::NotmuchMoreError;

pub struct Database {
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn open_ro(&self) -> Result<notmuch::Database, NotmuchMoreError> {
        Ok(notmuch::Database::open(
            &self.path,
            notmuch::DatabaseMode::ReadOnly,
        )?)
    }

    pub fn open_rw(&self) -> Result<notmuch::Database, NotmuchMoreError> {
        Ok(notmuch::Database::open(
            &self.path,
            notmuch::DatabaseMode::ReadWrite,
        )?)
    }
}

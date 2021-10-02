use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmailError {
    #[error(transparent)]
    NotmuchMoreError(#[from] notmuch_more::NotmuchMoreError),
}

impl From<AmailError> for tauri::InvokeError {
    fn from(e: AmailError) -> tauri::InvokeError {
        Self::from(format!("{}", e))
    }
}

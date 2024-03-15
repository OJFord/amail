use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmailError {
    #[error(transparent)]
    EmlParseError(#[from] notmuch_more::parse::EmlParseError),
    #[error(transparent)]
    NotmuchMoreError(#[from] notmuch_more::NotmuchMoreError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<AmailError> for tauri::ipc::InvokeError {
    fn from(e: AmailError) -> tauri::ipc::InvokeError {
        Self::from(format!("{e}"))
    }
}

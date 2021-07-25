use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmailError {
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    NotMuchError(#[from] notmuch::Error),
    #[error(transparent)]
    ParseError(#[from] mailparse::MailParseError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<AmailError> for tauri::InvokeError {
    fn from(e: AmailError) -> tauri::InvokeError {
        Self::from(format!("{}", e))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmlParseError {
    pub id: Option<String>,
    pub reason: String,
    pub within: Option<String>,
}

impl EmlParseError {
    pub fn new() -> Self {
        Self {
            id: None,
            reason: "Unknown".into(),
            within: None,
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = reason.into();
        self
    }

    pub fn within(mut self, within: &str) -> Self {
        self.within = Some(within.into());
        self
    }
}

impl<O: notmuch::MessageOwner> From<&notmuch::Message<'_, O>> for EmlParseError {
    fn from(m: &notmuch::Message<O>) -> Self {
        Self::new().id(m.id().into())
    }
}

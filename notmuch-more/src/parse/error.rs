use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Debug, Default, Error, Serialize, Deserialize)]
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

impl std::fmt::Display for EmlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let unknown_eml: String = "email".into();
        let eml = self.id.as_ref().unwrap_or(&unknown_eml);
        write!(
            f,
            "Error parsing {}: {}",
            self.within
                .as_ref()
                .map(|w| format!("{eml}'s {w}"))
                .unwrap_or_else(|| eml.into()),
            self.reason
        )
    }
}

impl From<&notmuch::Message> for EmlParseError {
    fn from(m: &notmuch::Message) -> Self {
        Self::new().id(m.id().into())
    }
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

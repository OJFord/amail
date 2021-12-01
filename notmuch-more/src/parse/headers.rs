use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

use chrono::DateTime;
use notmuch::Message;
use notmuch::MessageOwner;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use super::addresses::parse_address_header;
use super::addresses::parse_optional_address_list_header;
use super::EmlAddr;
use super::EmlParseError;
use super::Mailbox;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EmlMeta {
    pub cc: Option<Vec<EmlAddr>>,
    pub bcc: Option<Vec<EmlAddr>>,
    pub from: Vec<Mailbox>,
    pub id: String,
    pub id_thread: String,
    pub received_by: Option<Mailbox>,
    pub references: Option<String>,
    pub reply_to: Option<Vec<EmlAddr>>,
    pub sender: Option<Mailbox>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
    pub to: Option<Vec<EmlAddr>>,
    pub timestamp: i64,
}

pub(crate) fn parse_header<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<Option<String>, EmlParseError> {
    match eml.header(header) {
        Ok(Some(h)) => Ok(Some(h.into())),
        Ok(None) => Ok(None),
        Err(e) => Err(EmlParseError::from(eml)
            .within(header)
            .reason(&e.to_string())),
    }
}

pub(crate) fn must_parse_header<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<String, EmlParseError> {
    match parse_header(eml, header)? {
        Some(h) => Ok(h),
        None => Err(EmlParseError::from(eml).within(header).reason("Missing")),
    }
}

impl<'o, O: MessageOwner> TryFrom<&Message<'o, O>> for EmlMeta {
    type Error = EmlParseError;

    fn try_from(eml: &Message<O>) -> Result<Self, Self::Error> {
        Ok(EmlMeta {
            bcc: parse_optional_address_list_header(eml, "Bcc")?,

            cc: parse_optional_address_list_header(eml, "Cc")?,

            from: parse_address_header(eml, &must_parse_header(eml, "From")?)?
                .iter()
                .map(Mailbox::try_from)
                .collect::<Result<_, _>>()?,

            id: eml.id().to_string(),

            id_thread: eml.thread_id().to_string(),

            received_by: match parse_header(eml, "Received")? {
                Some(h) => {
                    let rx = Regex::new(r"for ([^\s]+@[^\s]+\.[^\s]+);")
                        .map_err(|e| {
                            EmlParseError::from(eml)
                                .reason(&format!("Building `Received` regex: {}", e))
                        })?
                        .captures(&h)
                        .map(|m| m.get(1).unwrap().as_str());

                    match rx {
                        Some(a) => parse_address_header(eml, a)
                            .map(|a| a.extract_single_info())?
                            .map(|s| Mailbox::from(&s)),
                        None => None,
                    }
                }
                None => None,
            },

            references: parse_header(eml, "References")?,

            reply_to: parse_optional_address_list_header(eml, "Reply-To")?,

            sender: match parse_header(eml, "Sender")? {
                Some(h) => parse_address_header(eml, &h)
                    .map(|a| a.extract_single_info())?
                    .map(|s| Mailbox::from(&s)),
                None => None,
            },

            subject: parse_header(eml, "Subject")?,

            tags: eml.tags().collect(),

            to: parse_optional_address_list_header(eml, "To")?,

            timestamp: eml.date(),
        })
    }
}

pub(crate) type Rfc5322Fields = HashMap<String, String>;

impl TryInto<EmlMeta> for Rfc5322Fields {
    type Error = EmlParseError;

    fn try_into(self) -> Result<EmlMeta, Self::Error> {
        Ok(EmlMeta {
            bcc: Some(
                mailparse::addrparse(self.get("Bcc").ok_or_else(EmlParseError::new)?)
                    .map_err(|e| Self::Error::new().reason(&e.to_string()))?
                    .iter()
                    .map(|a| Mailbox::try_from(a).map(EmlAddr::Single))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            cc: Some(
                mailparse::addrparse(self.get("Cc").ok_or_else(EmlParseError::new)?)
                    .map_err(|e| Self::Error::new().reason(&e.to_string()))?
                    .iter()
                    .map(|a| Mailbox::try_from(a).map(EmlAddr::Single))
                    .collect::<Result<_, _>>()?,
            ),
            from: mailparse::addrparse(self.get("From").ok_or_else(EmlParseError::new)?)
                .map_err(|e| Self::Error::new().reason(&e.to_string()))?
                .iter()
                .map(Mailbox::try_from)
                .collect::<Result<_, _>>()?,
            id: "".into(),
            id_thread: "".into(),
            received_by: None,
            references: self.get("References").cloned(),
            reply_to: {
                let addrs =
                    mailparse::addrparse(self.get("Reply-To").ok_or_else(EmlParseError::new)?)
                        .map_err(|e| Self::Error::new().reason(&e.to_string()))?;

                match addrs.iter().count() {
                    0 => None,
                    _ => Some(
                        addrs
                            .iter()
                            .map(EmlAddr::try_from)
                            .collect::<Result<_, _>>()?,
                    ),
                }
            },
            sender: {
                let addrs =
                    mailparse::addrparse(self.get("Sender").ok_or_else(EmlParseError::new)?)
                        .map_err(|e| Self::Error::new().reason(&e.to_string()))?;

                match addrs.iter().count() {
                    0 => Ok(None),
                    1 => Ok(Some(Mailbox::try_from(&addrs[0])?)),
                    _ => Err(Self::Error::new().reason("Too many Senders")),
                }?
            },
            subject: self.get("Subject").cloned(),
            tags: vec![],
            timestamp: DateTime::parse_from_rfc2822(self.get("Date").unwrap_or(&"".into()))
                .map(|d| d.timestamp())
                .unwrap_or(0),
            to: {
                let addrs = mailparse::addrparse(self.get("To").ok_or_else(EmlParseError::new)?)
                    .map_err(|e| Self::Error::new().reason(&e.to_string()))?;

                match addrs.iter().count() {
                    0 => None,
                    _ => Some(
                        addrs
                            .iter()
                            .map(EmlAddr::try_from)
                            .collect::<Result<_, _>>()?,
                    ),
                }
            },
        })
    }
}


use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use chrono::Utc;
use delegate::delegate;
use itertools::Itertools;
use notmuch::Message;
use notmuch::MessageOwner;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use super::addresses::parse_address_header;
use super::addresses::parse_optional_address_list_header;
use super::parse_address;
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
    pub in_reply_to: Option<String>,
    pub received_by: Option<Mailbox>,
    pub references: Option<String>,
    pub reply_to: Option<Vec<EmlAddr>>,
    pub sender: Option<Mailbox>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
    pub to: Option<Vec<EmlAddr>>,
    pub timestamp: i64,
}

impl EmlMeta {
    pub fn destinations(&self) -> Result<Vec<String>, EmlParseError> {
        Rfc5322Fields::from(self).destinations()
    }

    pub fn resolve_reply_to(&self) -> Result<String, EmlParseError> {
        Rfc5322Fields::from(self).resolve_reply_to()
    }

    pub fn resolve_sender(&self) -> Result<String, EmlParseError> {
        Rfc5322Fields::from(self).resolve_sender()
    }
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

            in_reply_to: parse_header(eml, "In-Reply-To")?,

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

pub(crate) struct Rfc5322Fields(HashMap<String, String>);

impl<const N: usize> From<[(String, String); N]> for Rfc5322Fields {
    fn from(arr: [(String, String); N]) -> Self {
        Self(HashMap::from(arr))
    }
}

impl Rfc5322Fields {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn cc(&mut self, cc: &[EmlAddr]) -> &mut Self {
        self.insert("Cc".into(), cc.iter().map(String::from).join(","));
        self
    }

    pub fn bcc(&mut self, bcc: &[EmlAddr]) -> &mut Self {
        self.insert("Bcc".into(), bcc.iter().map(String::from).join(","));
        self
    }

    pub fn date<Tz: chrono::TimeZone>(&mut self, date: &DateTime<Tz>) -> &mut Self
    where
        Tz::Offset: std::fmt::Display,
    {
        self.insert("Date".into(), date.to_rfc2822());
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_addr(&mut self, from: &[Mailbox]) -> &mut Self {
        self.insert("From".into(), from.iter().map(String::from).join(","));
        self
    }

    pub fn in_reply_to(&mut self, in_reply_to: &str) -> &mut Self {
        self.insert("In-Reply-To".into(), in_reply_to.into());
        self
    }

    pub fn message_id(&mut self, message_id: &str) -> &mut Self {
        self.insert("Message-ID".into(), message_id.into());
        self
    }

    pub fn references(&mut self, refs: &str) -> &mut Self {
        self.insert("References".into(), refs.into());
        self
    }

    pub fn reply_to(&mut self, reply_to: &[EmlAddr]) -> &mut Self {
        self.insert(
            "Reply-To".into(),
            reply_to.iter().map(String::from).join(","),
        );
        self
    }

    pub fn sender(&mut self, sender: &Mailbox) -> &mut Self {
        self.insert("Sender".into(), sender.into());
        self
    }

    pub fn subject(&mut self, subject: &str) -> &mut Self {
        self.insert("Subject".into(), subject.into());
        self
    }

    pub fn to(&mut self, to: &[EmlAddr]) -> &mut Self {
        self.insert("To".into(), to.iter().map(String::from).join(","));
        self
    }

    delegate! {
        to self.0 {
            pub fn get(&self, k: &str) -> Option<&String>;
            pub fn iter(&self) -> std::collections::hash_map::Iter<String, String>;
            pub fn insert(&mut self, k: String, v: String) ->  Option<String>;
        }
    }

    pub fn format_fields(&self) -> String {
        itertools::sorted(self.iter())
            .map(|(k, v)| match k.as_str() {
                "Bcc" => "Bcc:".into(),
                _ => format!("{}: {}", k, v),
            })
            .join("\r\n")
    }

    pub fn format_message(&self, body: &str, boundary: &str) -> String {
        format!(
            "{}\r\nContent-Type: multipart/mixed; boundary={}\r\n\r\n{}\r\n--{}--",
            self.format_fields(),
            boundary,
            body,
            boundary,
        )
    }

    pub fn format_message_id_for_destination(&self, dest: &str) -> String {
        if let Some(curr) = self.get("Message-ID") {
            if let Ok(re) = Regex::new(r"^<(?P<t>.*)\.(?P<id>.*)\.(?P<d>.*)>$") {
                return re.replace(curr, format!("<$t.$id.{}>", dest)).into();
            }
        }

        let dt = self
            .get("Date")
            .map(|d| {
                DateTime::parse_from_rfc2822(d).unwrap_or_else(|_| {
                    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
                })
            })
            .unwrap_or_else(|| Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()));
        format!("<{}.{}.{}>", dt.timestamp(), "unknown", dest)
    }

    pub fn resolve_reply_to(&self) -> Result<String, EmlParseError> {
        if let Some(reply_to) = self.get("Reply-To") {
            return Ok(reply_to.clone());
        }

        self.get("From")
            .cloned()
            .ok_or_else(|| EmlParseError::new().within("From").reason("Missing header"))
    }

    pub fn resolve_sender(&self) -> Result<String, EmlParseError> {
        if let Some(sender) = self.get("Sender") {
            let mboxes = parse_address(sender).map_err(|e| {
                EmlParseError::new().reason(&format!("Failed to parse address: {}", e))
            })?;
            if mboxes.len() != 1 {
                return Err(
                    EmlParseError::new().reason("Must have exactly one Sender (if present)")
                );
            }
            return Ok(mboxes[0].address.clone());
        }

        let mboxes = parse_address(
            self.get("From")
                .ok_or_else(|| EmlParseError::new().reason("Missing From header"))?,
        )
        .map_err(|e| EmlParseError::new().reason(&format!("Failed to parse address: {}", e)))?;
        if mboxes.len() > 1 {
            return Err(EmlParseError::new().reason("Must set Sender if multiple From addresses"));
        }
        if mboxes.is_empty() {
            return Err(EmlParseError::new().reason("Missing From address"));
        }

        Ok(mboxes[0].address.clone())
    }

    pub fn destinations(&self) -> Result<Vec<String>, EmlParseError> {
        let mboxes_from = |f| {
            self.get(f)
                .map(|a| {
                    parse_address(a).map_err(|e| {
                        EmlParseError::new().reason(&format!("Failed to parse address: {}", e))
                    })
                })
                .unwrap_or_else(|| Ok(vec![]))
        };

        Ok(mboxes_from("To")?
            .iter()
            .chain(mboxes_from("Cc")?.iter())
            .chain(mboxes_from("Bcc")?.iter())
            .map(|m| m.address.clone())
            .collect())
    }
}

impl From<&EmlMeta> for Rfc5322Fields {
    fn from(meta: &EmlMeta) -> Self {
        let mut fields = Self::new();

        if let Some(cc) = &meta.cc {
            fields.cc(cc);
        }

        if let Some(bcc) = &meta.bcc {
            fields.bcc(bcc);
        }

        if let Some(in_reply_to) = &meta.in_reply_to {
            fields.in_reply_to(in_reply_to);
        }

        if let Some(references) = &meta.references {
            fields.references(references);
        }

        if let Some(reply_to) = &meta.reply_to {
            fields.reply_to(reply_to);
        }

        if let Some(sender) = &meta.sender {
            fields.sender(sender);
        }

        if let Some(subject) = &meta.subject {
            fields.subject(subject);
        }

        if let Some(to) = &meta.to {
            fields.to(to);
        }

        fields.from_addr(&meta.from);
        fields.message_id(&fields.format_message_id_for_destination(&{
            let maybe_dests = fields.destinations();
            match maybe_dests {
                Ok(dests) => match dests.len() {
                    0 => "@unknown".into(),
                    1 => dests[0].clone(),
                    _ => "@multiple".into(),
                },
                _ => "@unknown".into(),
            }
        }));

        fields.date(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(meta.timestamp, 0).unwrap(),
            Utc,
        ));

        fields
    }
}

impl TryInto<EmlMeta> for Rfc5322Fields {
    type Error = EmlParseError;

    fn try_into(self) -> Result<EmlMeta, Self::Error> {
        Ok(EmlMeta {
            bcc: match self.get("Bcc") {
                Some(bcc) => Some(
                    mailparse::addrparse(bcc)
                        .map_err(|e| Self::Error::new().within("Bcc").reason(&e.to_string()))?
                        .iter()
                        .map(|a| Mailbox::try_from(a).map(EmlAddr::Single))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                _ => None,
            },
            cc: match self.get("Cc") {
                Some(cc) => Some(
                    mailparse::addrparse(cc)
                        .map_err(|e| Self::Error::new().within("Cc").reason(&e.to_string()))?
                        .iter()
                        .map(|a| Mailbox::try_from(a).map(EmlAddr::Single))
                        .collect::<Result<_, _>>()?,
                ),
                _ => None,
            },
            from: match self.get("From") {
                Some(from) => Ok(mailparse::addrparse(from)
                    .map_err(|e| Self::Error::new().within("From").reason(&e.to_string()))?
                    .iter()
                    .map(Mailbox::try_from)
                    .collect::<Result<_, _>>()?),
                _ => Err(Self::Error::new().within("From").reason("Missing header")),
            }?,
            id: "".into(),
            id_thread: "".into(),
            in_reply_to: self.get("In-Reply-To").cloned(),
            received_by: None,
            references: self.get("References").cloned(),
            reply_to: {
                let addrs = mailparse::addrparse(self.get("Reply-To").unwrap_or(&"".into()))
                    .map_err(|e| Self::Error::new().within("Reply-To").reason(&e.to_string()))?;

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
                let addrs = mailparse::addrparse(self.get("Sender").unwrap_or(&"".into()))
                    .map_err(|e| Self::Error::new().within("Sender").reason(&e.to_string()))?;

                match addrs.iter().count() {
                    0 => Ok(None),
                    1 => Ok(Some(Mailbox::try_from(&addrs[0])?)),
                    _ => Err(Self::Error::new()
                        .within("Sender")
                        .reason("Too many Senders")),
                }?
            },
            subject: self.get("Subject").cloned(),
            tags: vec![],
            timestamp: DateTime::parse_from_rfc2822(self.get("Date").unwrap_or(&"".into()))
                .map(|d| d.timestamp())
                .unwrap_or(0),
            to: {
                let addrs = mailparse::addrparse(self.get("To").unwrap_or(&"".into()))
                    .map_err(|e| Self::Error::new().within("To").reason(&e.to_string()))?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_rfc5322_fields() {
        assert_eq!(
            Rfc5322Fields::from([
                ("Subject".into(), "blah".into()),
                ("To".into(), "foo@bar.com".into()),
            ])
            .format_fields(),
            "Subject: blah\r\nTo: foo@bar.com",
        )
    }

    #[test]
    fn rfc5322_fields_bcc_blind() {
        assert_eq!(
            Rfc5322Fields::from([
                ("Bcc".into(), "foo@bar.com".into()),
                ("To".into(), "bar@foo.com".into()),
            ])
            .format_fields(),
            "Bcc:\r\nTo: bar@foo.com",
        )
    }
}

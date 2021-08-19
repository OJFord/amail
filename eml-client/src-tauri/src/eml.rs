use std::convert::TryFrom;

use mailparse::MailAddr;
use notmuch::Message;
use notmuch::MessageOwner;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::error::EmlParseError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mailbox {
    pub name: String,
    pub address: String,
}

impl From<&mailparse::SingleInfo> for Mailbox {
    fn from(s: &mailparse::SingleInfo) -> Self {
        Mailbox {
            name: s
                .display_name
                .to_owned()
                .unwrap_or_else(|| String::from("")),
            address: s.addr.to_owned(),
        }
    }
}

impl TryFrom<&MailAddr> for Mailbox {
    type Error = EmlParseError;

    fn try_from(addr: &MailAddr) -> Result<Self, Self::Error> {
        match addr {
            MailAddr::Single(s) => Ok(Self::from(s)),
            _ => Err(EmlParseError::new().reason("Expected single mailbox")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmlAddr {
    Single(Mailbox),
    Group { name: String, members: Vec<Mailbox> },
}

impl TryFrom<&MailAddr> for EmlAddr {
    type Error = EmlParseError;

    fn try_from(addr: &MailAddr) -> Result<Self, Self::Error> {
        match addr {
            MailAddr::Group(g) => Ok(Self::Group {
                name: g.group_name.to_owned(),
                members: g.addrs.iter().map(Mailbox::from).collect(),
            }),
            MailAddr::Single(s) => Ok(Self::Single(Mailbox::from(s))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

fn parse_header<O: MessageOwner>(
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

fn must_parse_header<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<String, EmlParseError> {
    match parse_header(eml, header)? {
        Some(h) => Ok(h),
        None => Err(EmlParseError::from(eml).within(header).reason("Missing")),
    }
}

fn parse_address<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<mailparse::MailAddrList, EmlParseError> {
    mailparse::addrparse(header).map_err(|e| {
        EmlParseError::from(eml)
            .within(header)
            .reason(&e.to_string())
    })
}

fn parse_optional_address_list<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<Option<Vec<EmlAddr>>, EmlParseError> {
    match parse_header(eml, header)? {
        Some(h) => Ok(Some(
            parse_address(eml, &h)?
                .iter()
                .map(EmlAddr::try_from)
                .collect::<Result<_, _>>()?,
        )),
        None => Ok(None),
    }
}

impl<'o, O: MessageOwner> TryFrom<&Message<'o, O>> for EmlMeta {
    type Error = EmlParseError;

    fn try_from(eml: &Message<O>) -> Result<Self, Self::Error> {
        Ok(EmlMeta {
            bcc: parse_optional_address_list(eml, "Bcc")?,

            cc: parse_optional_address_list(eml, "Cc")?,

            from: parse_address(eml, &must_parse_header(eml, "From")?)?
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
                        Some(a) => parse_address(eml, a)
                            .map(|a| a.extract_single_info())?
                            .map(|s| Mailbox::from(&s)),
                        None => None,
                    }
                }
                None => None,
            },

            references: parse_header(eml, "References")?,

            reply_to: parse_optional_address_list(eml, "Reply-To")?,

            sender: match parse_header(eml, "Sender")? {
                Some(h) => parse_address(eml, &h)
                    .map(|a| a.extract_single_info())?
                    .map(|s| Mailbox::from(&s)),
                None => None,
            },

            subject: parse_header(eml, "Subject")?,

            tags: eml.tags().collect(),

            to: parse_optional_address_list(eml, "To")?,

            timestamp: eml.date(),
        })
    }
}

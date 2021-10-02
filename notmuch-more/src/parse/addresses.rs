use std::convert::TryFrom;

use mailparse::MailAddr;
use notmuch::Message;
use notmuch::MessageOwner;
use serde::Deserialize;
use serde::Serialize;

use super::headers::parse_header;
use super::EmlParseError;

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

pub(crate) fn parse_address<O: MessageOwner>(
    eml: &Message<'_, O>,
    header: &str,
) -> Result<mailparse::MailAddrList, EmlParseError> {
    mailparse::addrparse(header).map_err(|e| {
        EmlParseError::from(eml)
            .within(header)
            .reason(&e.to_string())
    })
}

pub(crate) fn parse_optional_address_list<O: MessageOwner>(
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

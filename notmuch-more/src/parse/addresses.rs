use std::convert::TryFrom;
use std::fmt::Display;

use mailparse::MailAddr;
use notmuch::Message;
use serde::Deserialize;
use serde::Serialize;

use super::headers::parse_header;
use super::EmlParseError;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Mailbox {
    pub name: String,
    pub address: String,
}

impl TryFrom<&EmlAddr> for Mailbox {
    type Error = EmlParseError;

    fn try_from(addr: &EmlAddr) -> Result<Self, Self::Error> {
        match addr {
            EmlAddr::Single(a) => Ok(Self {
                name: a.name.clone(),
                address: a.address.clone(),
            }),
            EmlAddr::Group { name, .. } => {
                Err(EmlParseError::new().reason(&format!("Not a single mailbox: {name}")))
            }
        }
    }
}

impl From<&Mailbox> for String {
    fn from(mbox: &Mailbox) -> Self {
        format!("\"{}\" <{}>", mbox.name, mbox.address)
    }
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

impl From<&EmlAddr> for String {
    fn from(addr: &EmlAddr) -> Self {
        match addr {
            EmlAddr::Single(mbox) => String::from(mbox),
            EmlAddr::Group { name, members } => {
                format!(
                    "{}: {};",
                    name,
                    members
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl Display for EmlAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
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

pub(crate) fn parse_address_header(
    eml: &Message,
    header: &str,
) -> Result<mailparse::MailAddrList, EmlParseError> {
    mailparse::addrparse(header).map_err(|e| {
        EmlParseError::from(eml)
            .within(header)
            .reason(&e.to_string())
    })
}

pub(crate) fn parse_optional_address_list_header(
    eml: &Message,
    header: &str,
) -> Result<Option<Vec<EmlAddr>>, EmlParseError> {
    match parse_header(eml, header)? {
        Some(h) => Ok(Some(
            parse_address_header(eml, &h)?
                .iter()
                .map(EmlAddr::try_from)
                .collect::<Result<_, _>>()?,
        )),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_mailbox_string() {
        let mbox = Mailbox {
            name: "Gregory House".into(),
            address: "diagnostics@pph.com".into(),
        };

        assert_eq!(
            &String::from(&mbox),
            "\"Gregory House\" <diagnostics@pph.com>"
        );
    }

    #[test]
    fn single_addr_string() {
        let single = EmlAddr::Single(Mailbox {
            name: "Gregory House".into(),
            address: "diagnostics@pph.com".into(),
        });

        assert_eq!(
            &String::from(&single),
            "\"Gregory House\" <diagnostics@pph.com>",
        );
    }

    #[test]
    fn group_addr_string() {
        let group = EmlAddr::Group {
            name: "Docs".into(),
            members: vec![
                Mailbox {
                    name: "Gregory House".into(),
                    address: "diagnostics@pph.com".into(),
                },
                Mailbox {
                    name: "Lisa Cuddy".into(),
                    address: "ceo@pph.com".into(),
                },
            ],
        };

        assert_eq!(
            &String::from(&group),
            "Docs: \"Gregory House\" <diagnostics@pph.com>, \"Lisa Cuddy\" <ceo@pph.com>;",
        );
    }
}

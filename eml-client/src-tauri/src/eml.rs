use std::convert::TryFrom;

use anyhow::anyhow;
use itertools::Itertools;
use mailparse::MailAddr;
use notmuch::Message;
use notmuch::MessageOwner;
use serde::Deserialize;
use serde::Serialize;

use crate::AmailError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmlAddr {
    Single { name: String, address: String },
    Group { name: String, members: Vec<EmlAddr> },
}

impl From<&MailAddr> for EmlAddr {
    fn from(addr: &MailAddr) -> Self {
        match addr {
            MailAddr::Group(g) => Self::Group {
                name: g.group_name.to_owned(),
                members: g
                    .addrs
                    .iter()
                    .map(|s| Self::from(&MailAddr::Single(s.to_owned())))
                    .collect(),
            },
            MailAddr::Single(s) => Self::Single {
                name: s
                    .display_name
                    .to_owned()
                    .unwrap_or_else(|| String::from("")),
                address: s.addr.to_owned(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmlMeta {
    pub from: EmlAddr,
    pub id: String,
    pub id_thread: String,
    pub subject: String,
    pub tags: Vec<String>,
    pub to: Vec<EmlAddr>,
    pub timestamp: i64,
}

impl<'o, O> TryFrom<&Message<'o, O>> for EmlMeta
where
    O: MessageOwner + 'o,
{
    type Error = AmailError;

    fn try_from(eml: &Message<O>) -> Result<Self, Self::Error> {
        Ok(EmlMeta {
            from: mailparse::addrparse(
                &eml.header("From")?.ok_or_else(|| anyhow!("Missing From"))?,
            )?
            .into_inner()
            .iter()
            .map(EmlAddr::from)
            .collect::<Vec<EmlAddr>>()
            .into_iter()
            .exactly_one()
            .map_err(|e| anyhow!("Expected exactly one From address: {:?}", e))?,

            id: eml.id().to_string(),
            id_thread: eml.thread_id().to_string(),

            subject: eml
                .header("Subject")?
                .ok_or_else(|| anyhow!("Missing subject"))?
                .into(),

            tags: eml.tags().collect(),

            to: mailparse::addrparse(&eml.header("To")?.ok_or_else(|| anyhow!("Missing From"))?)?
                .into_inner()
                .iter()
                .map(EmlAddr::from)
                .collect(),

            timestamp: eml.date(),
        })
    }
}

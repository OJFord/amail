use std::convert::TryFrom;

use anyhow::anyhow;
use mailparse::MailAddr;
use notmuch::Message;
use notmuch::MessageOwner;
use serde::Serialize;

#[derive(Serialize)]
pub struct EmlMeta {
    author: String,
    subject: String,
    timestamp: i64,
}

impl<'o, O> TryFrom<&Message<'o, O>> for EmlMeta
where
    O: MessageOwner + 'o,
{
    type Error = crate::AmailError;

    fn try_from(eml: &Message<O>) -> Result<Self, Self::Error> {
        Ok(EmlMeta {
            author: eml
                .header("From")?
                .ok_or_else(|| anyhow!("Missing From"))
                .and_then(|ref f| {
                    Ok(mailparse::addrparse_header(
                        &mailparse::parse_header(f.as_bytes())?.0,
                    )?)
                })
                .map(|a| {
                    a.into_inner()
                        .iter()
                        .map(|a| match a {
                            MailAddr::Group(g) => g.group_name.to_owned(),
                            MailAddr::Single(s) => s
                                .display_name
                                .to_owned()
                                .unwrap_or_else(|| s.addr.to_owned()),
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                })?,

            subject: eml
                .header("Subject")?
                .ok_or_else(|| anyhow!("Missing subject"))?
                .into(),

            timestamp: eml.date(),
        })
    }
}

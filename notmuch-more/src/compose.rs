use std::convert::TryInto;

use anyhow::anyhow;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use itertools::Itertools;
use notmuch::Database;
use serde::Serialize;

use crate::parse;
use crate::NotmuchMoreError;
use parse::EmlAddr;
use parse::EmlBody;
use parse::EmlMeta;
use parse::EmlParseError;
use parse::Rfc5322Fields;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReplyTemplate {
    pub meta: EmlMeta,
    pub body: String,
}

fn format_message_id(meta: &EmlMeta) -> String {
    format!(
        "<{}.{}.{}>",
        Utc::now().timestamp(),
        meta.id_thread,
        meta.to
            .as_ref()
            .unwrap_or(&vec![])
            .last()
            .and_then(|to| match to {
                EmlAddr::Single(mbox) => Some(&mbox.address),
                EmlAddr::Group { members, .. } => members.last().map(|mbox| &mbox.address),
            })
            .unwrap_or(&String::from("@unknown"))
    )
}

fn template_body(meta: &EmlMeta, body: &EmlBody) -> String {
    format!(
        "\r\n\r\nOn {}, {} wrote:\r\n{}",
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(meta.timestamp, 0), Utc)
            .to_rfc2822(),
        meta.from
            .last()
            .map(String::from)
            .unwrap_or_else(|| "".into()),
        parse::plaintext(body).unwrap_or_else(|| "[no plaintext]".into()),
    )
}

pub fn template_reply(db: &Database, id: String) -> Result<ReplyTemplate, NotmuchMoreError> {
    let (reply_to_meta, msg) = parse::parse_eml(db, id)?;

    Ok(ReplyTemplate {
        meta: Rfc5322Fields::from([
            ("Message-ID".into(), format_message_id(&reply_to_meta)),
            ("Date".into(), Local::now().to_rfc2822()),
            (
                "From".into(),
                reply_to_meta
                    .to
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(String::from)
                    .join(","),
            ),
            (
                "To".into(),
                match reply_to_meta.reply_to.as_ref() {
                    Some(to) => to.iter().map(String::from).collect::<Vec<String>>(),
                    None => reply_to_meta.from.iter().map(String::from).collect(),
                }
                .join(","),
            ),
            (
                "Cc".into(),
                reply_to_meta
                    .cc
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            ("Bcc".into(), "".into()),
            ("In-Reply-To".into(), reply_to_meta.id.clone()),
            (
                "References".into(),
                format!(
                    "{} {}",
                    reply_to_meta
                        .references
                        .as_ref()
                        .unwrap_or(&String::from("")),
                    reply_to_meta.id.clone()
                ),
            ),
            (
                "Subject".into(),
                reply_to_meta
                    .subject
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "".into()),
            ),
        ])
        .try_into()
        .map_err(|e: EmlParseError| anyhow!("Parsing failed: {}", e.reason))?,
        body: template_body(&reply_to_meta, &msg),
    })
}

pub fn rfc5322_fields(fields: &Rfc5322Fields) -> String {
    itertools::sorted(fields.iter())
        .map(|(k, v)| match k.as_str() {
            "Bcc" => "Bcc:".into(),
            _ => format!("{}: {}", k, v),
        })
        .join("\r\n")
}

pub fn rfc5322_body(body: &str) -> String {
    body.lines()
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(78)
                .map(|l| l.iter().collect::<String>())
                .join("\r\n")
        })
        .join("\r\n")
}

pub fn rfc5322_message(fields: &Rfc5322Fields, body: &str) -> String {
    format!("{}\r\n\r\n{}", rfc5322_fields(fields), rfc5322_body(body))
}

pub fn rfc5322_sender(fields: &Rfc5322Fields) -> Result<String, NotmuchMoreError> {
    if let Some(sender) = fields.get("Sender") {
        let mboxes = parse::parse_address(sender)?;
        if mboxes.len() != 1 {
            return Err(NotmuchMoreError::Other(anyhow!(
                "Must have exactly one Sender (if present)"
            )));
        }
        return Ok(mboxes[0].address.clone());
    }

    let mboxes = parse::parse_address(
        fields
            .get("From")
            .ok_or_else(|| anyhow!("Missing From header"))?,
    )?;
    if mboxes.len() > 1 {
        return Err(NotmuchMoreError::Other(anyhow!(
            "Must set Sender if multiple From addresses"
        )));
    }
    if mboxes.is_empty() {
        return Err(NotmuchMoreError::Other(anyhow!("Missing From address")));
    }

    Ok(mboxes[0].address.clone())
}

pub fn rfc5322_destinations(fields: &Rfc5322Fields) -> Result<Vec<String>, NotmuchMoreError> {
    let mboxes_from = |f| {
        fields
            .get(f)
            .map(|a| parse::parse_address(a))
            .unwrap_or_else(|| Ok(vec![]))
    };

    Ok(mboxes_from("To")?
        .iter()
        .chain(mboxes_from("Cc")?.iter())
        .chain(mboxes_from("Bcc")?.iter())
        .map(|m| m.address.clone())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse::Mailbox;
    use std::default::Default;

    #[test]
    fn simple_body_template() {
        let meta = EmlMeta {
            from: vec![Mailbox {
                name: "Enid Blyton".into(),
                address: "enid@blyt.on".into(),
            }],
            timestamp: 1234567890,
            ..Default::default()
        };
        let body = EmlBody {
            content: "Five Write Some Rust".into(),
            mimetype: "text/plain".into(),
            ..Default::default()
        };

        assert_eq!(
            template_body(&meta, &body),
            "\r\n\r\nOn Fri, 13 Feb 2009 23:31:30 +0000, Enid Blyton <enid@blyt.on> wrote:\r\nFive Write Some Rust",
        );
    }
}

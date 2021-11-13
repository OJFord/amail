use itertools::Itertools;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use notmuch::Database;

use crate::parse;
use crate::NotmuchMoreError;
use parse::EmlAddr;
use parse::EmlBody;
use parse::EmlMeta;

pub struct ReplyTemplate {
    pub headers: HashMap<&'static str, String>,
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
        "\n\r\n\rOn {}, {} wrote:\n\r{}",
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
    let (meta, msg) = parse::parse_eml(db, id)?;
    Ok(ReplyTemplate {
        headers: HashMap::from([
            ("Message-ID", format_message_id(&meta)),
            ("Date", Local::now().to_rfc2822()),
            (
                "From",
                meta.to
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(String::from)
                    .join(","),
            ),
            (
                "To",
                match meta.reply_to.as_ref() {
                    Some(to) => to.iter().map(String::from).collect::<Vec<String>>(),
                    None => meta.from.iter().map(String::from).collect(),
                }
                .join(","),
            ),
            (
                "Cc",
                meta.cc
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            ("Bcc", "".into()),
            ("In-Reply-To", meta.id.clone()),
            (
                "References",
                format!(
                    "{} {}",
                    meta.references.as_ref().unwrap_or(&String::from("")),
                    meta.id.clone()
                ),
            ),
            (
                "Subject",
                meta.subject
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "".into()),
            ),
        ]),
        body: template_body(&meta, &msg),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse::Mailbox;
    use std::default::Default;

    #[test]
    fn simple_body() {
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
            "\n\r\n\rOn Fri, 13 Feb 2009 23:31:30 +0000, Enid Blyton <enid@blyt.on> wrote:\n\rFive Write Some Rust",
        );
    }
}

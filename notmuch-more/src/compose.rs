use std::convert::TryFrom;
use std::convert::TryInto;

use anyhow::anyhow;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use notmuch::Database;
use serde::Serialize;

use crate::parse;
use crate::NotmuchMoreError;
use parse::EmlAddr;
use parse::EmlBody;
use parse::EmlMeta;
use parse::Mailbox;
use parse::Rfc5322Fields;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReplyTemplate {
    pub meta: EmlMeta,
    pub body: String,
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

    let mut reply_fields = Rfc5322Fields::from(&reply_to_meta);
    reply_fields.date(&Local::now());
    reply_fields.message_id(
        &reply_fields.format_message_id_for_destination(
            &reply_fields
                .resolve_reply_to()
                .map_err(|e| anyhow!("Failed to parse: {}", e))?,
        ),
    );
    reply_fields.in_reply_to(&reply_to_meta.id);
    reply_fields.references(&format!(
        "{} {}",
        &reply_to_meta.references.as_ref().unwrap_or(&String::new()),
        &reply_to_meta.id
    ));

    if let Some(original_to) = &reply_to_meta.to {
        let from: Vec<Mailbox> = original_to
            .iter()
            .map(Mailbox::try_from)
            .collect::<Result<_, _>>()
            .map_err(|e| anyhow!("Failed to parse: {}", e))?;

        reply_fields.from_addr(&from);
    }

    let from_addrs = reply_to_meta
        .from
        .iter()
        .map(|m| EmlAddr::Single(m.clone()))
        .collect::<Vec<EmlAddr>>();
    reply_fields.to(match &reply_to_meta.reply_to {
        Some(ref original_reply_to) => original_reply_to,
        _ => &from_addrs,
    });

    Ok(ReplyTemplate {
        meta: reply_fields
            .try_into()
            .map_err(|e| anyhow!("Failed to parse: {}", e))?,
        body: template_body(&reply_to_meta, &msg),
    })
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

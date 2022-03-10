use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs;

use anyhow::anyhow;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use itertools::Itertools;
use notmuch::Database;
use regex::Regex;
use serde::Deserialize;
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Attachment {
    pub name: String,
    pub path: String,
}

fn format_part(
    boundary: &str,
    ctype: &str,
    ctencoding: &str,
    disposition: &str,
    content: &str,
) -> String {
    format!(
        "--{}\r\nContent-Type: {}\r\nContent-Transfer-Encoding: {}\r\nContent-Disposition: {}\r\n\r\n{}\r\n\r\n",
        boundary,
        ctype,
        ctencoding,
        disposition,
        content,
    )
}

fn format_body(body: &str) -> String {
    Regex::new(r"(^|[^\r])\n")
        .unwrap()
        // SHOULD limit to 78ch for readability
        .replace_all(&textwrap::fill(body, 78), "$1\r\n")
        .into()
}

fn format_attachment(content: &str) -> String {
    Regex::new(r"(^|[^\r])\n")
        .unwrap()
        // MUST limit to 998ch; since this is not human-readable content
        // there's no point further limiting to 78ch for readability.
        .replace_all(&textwrap::fill(content, 998), "$1\r\n")
        .into()
}

pub fn format_message(
    meta: &EmlMeta,
    body: String,
    attachments: Vec<Attachment>,
) -> Result<String, NotmuchMoreError> {
    let boundary = "amail-boundary";
    let mut parts: Vec<String> = vec![format_part(
        boundary,
        "text/plain; charset=utf-8",
        "8bit",
        "inline",
        &format_body(&body),
    )];

    for attachment in attachments {
        parts.push(format_part(
            boundary,
            mime_guess::from_path(&attachment.path)
                .first_or_octet_stream()
                .essence_str(),
            "base64",
            &format!("attachment; filename={}", attachment.name),
            &format_attachment(&base64::encode(fs::read(&attachment.path)?)),
        ));
    }

    Ok(Rfc5322Fields::from(meta).format_message(&parts.iter().join(""), boundary))
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
    println!("[TRACE] templating reply");
    let (reply_to_meta, msg) = parse::parse_eml(db, id)?;

    println!("[TRACE] building Rfc5322Fields");
    let mut reply_fields = Rfc5322Fields::new();
    reply_fields.subject(reply_to_meta.subject.as_deref().unwrap_or(""));
    if let Some(cc) = &reply_to_meta.cc {
        reply_fields.cc(cc);
    }

    reply_fields.date(&Local::now());
    reply_fields.message_id(
        &reply_fields.format_message_id_for_destination(
            &reply_to_meta
                .resolve_reply_to()
                .map_err(|e| anyhow!("Failed to parse: {}", e))?,
        ),
    );
    reply_fields.in_reply_to(&format!("<{}>", &reply_to_meta.id));
    reply_fields.references(&format!(
        "{} <{}>",
        &reply_to_meta.references.as_ref().unwrap_or(&String::new()),
        &reply_to_meta.id
    ));

    println!("[TRACE] swapping reply's from addr");
    if let Some(original_to) = &reply_to_meta.to {
        let from: Vec<Mailbox> = original_to
            .iter()
            .map(Mailbox::try_from)
            .collect::<Result<_, _>>()
            .map_err(|e| anyhow!("Failed to parse: {}", e))?;

        reply_fields.from_addr(&from);
    }

    println!("[TRACE] swapping reply's to addr");
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
    fn rfc5322_body_no_linebreak() {
        let body = ".".repeat(78);
        assert_eq!(format_body(&body), ".".repeat(78))
    }

    #[test]
    fn rfc5322_body_force_linebreak() {
        let body = ".".repeat(80);
        assert_eq!(
            format_body(&body),
            format!("{}\r\n{}", ".".repeat(78), ".".repeat(2)),
        )
    }

    #[test]
    fn rfc5322_body_with_extant_linebreaks() {
        let body = "hi there, yes, look:\r\n```\r\nfoo\r\n```\r\n\r\nMany thanks,";
        assert_eq!(format_body(body), body)
    }

    #[test]
    fn rfc5322_body_with_extant_and_new_linebreaks() {
        let body = format!(
            "hi there, yes, look:\r\n```\r\nfo{}\r\n```\r\n\r\nMany thanks,",
            "o".repeat(200)
        );
        assert_eq!(
            format_body(&body),
            format!(
                "hi there, yes, look:\r\n```\r\nfo{}\r\n{}\r\n{}\r\n```\r\n\r\nMany thanks,",
                "o".repeat(76),
                "o".repeat(78),
                "o".repeat(46),
            )
        )
    }

    #[test]
    fn rfc5322_body_no_break_word() {
        let body = format!("{} word word word.", ".".repeat(75));
        assert_eq!(
            format_body(&body),
            format!("{}\r\nword word word.", ".".repeat(75))
        )
    }

    #[test]
    fn rfc5322_body_crlf() {
        assert_eq!(format_body("\n"), "\r\n")
    }

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

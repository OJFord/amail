use std::convert::TryFrom;

use anyhow::anyhow;
use itertools::Itertools;
use notmuch::Database;

use crate::NotmuchMoreError;

mod addresses;
mod body;
mod error;
mod headers;

pub(crate) use headers::Rfc5322Fields;

pub use addresses::EmlAddr;
pub use addresses::Mailbox;
pub use body::EmlBody;
pub use error::EmlParseError;
pub use headers::EmlMeta;

pub fn parse_address(addr: &str) -> Result<Vec<Mailbox>, NotmuchMoreError> {
    let mboxes = mailparse::addrparse(addr)
        .map_err(|e| anyhow!("Failed to parse address {}: {}", addr, e))?
        .iter()
        .map(Mailbox::try_from)
        .collect::<Result<Vec<Mailbox>, EmlParseError>>();

    Ok(mboxes.map_err(|e| anyhow!("Failed to parse address: {:?}", e))?)
}

pub fn parse_eml(db: &Database, id: String) -> Result<(EmlMeta, EmlBody), NotmuchMoreError> {
    println!("Opening id:{}", id);
    let msg = db
        .find_message(&id)?
        .ok_or_else(|| anyhow!("Message {} not found", id))?;
    let contents = &std::fs::read(msg.filename())?;

    println!("Parsing id:{}", id);
    let meta =
        EmlMeta::try_from(&msg).map_err(|e| anyhow!("Could not parse {}: {}", id, e.reason))?;
    let body = body::parse_body_part(&mailparse::parse_mail(contents)?)?;
    Ok((meta, body))
}

pub fn plaintext(eml: &EmlBody) -> Option<String> {
    let mut candidates: Vec<&EmlBody> = eml.alternatives.iter().collect();
    candidates.push(eml);
    candidates
        .iter()
        .find(|&e| e.mimetype == "text/plain")
        .map(|e| e.content.clone())
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
        let mboxes = parse_address(sender)?;
        if mboxes.len() != 1 {
            return Err(NotmuchMoreError::Other(anyhow!(
                "Must have exactly one Sender (if present)"
            )));
        }
        return Ok(mboxes[0].address.clone());
    }

    let mboxes = parse_address(
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
            .map(|a| parse_address(a))
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
    use std::collections::HashMap;
    use std::default::Default;

    #[test]
    fn no_plaintext() {
        let eml = EmlBody {
            alternatives: vec![],
            mimetype: "not/plain".into(),
            ..Default::default()
        };

        assert_eq!(plaintext(&eml), None);
    }

    #[test]
    fn top_level_plaintext() {
        let eml = EmlBody {
            alternatives: vec![EmlBody {
                ..Default::default()
            }],
            content: "plaintext!".into(),
            mimetype: "text/plain".into(),
            ..Default::default()
        };

        assert_eq!(plaintext(&eml), Some("plaintext!".into()));
    }

    #[test]
    fn nested_plaintext() {
        let eml = EmlBody {
            alternatives: vec![EmlBody {
                content: "plaintext!".into(),
                mimetype: "text/plain".into(),
                ..Default::default()
            }],
            content: "not plaintext".into(),
            ..Default::default()
        };

        assert_eq!(plaintext(&eml), Some("plaintext!".into()));
    }

    #[test]
    fn simple_addr() {
        let addr = "Foo Bar <foo@bar.com>";

        let mboxes = parse_address(addr).unwrap();

        assert_eq!(
            mboxes[0],
            Mailbox {
                name: "Foo Bar".into(),
                address: "foo@bar.com".into(),
            }
        );
    }

    #[test]
    fn simple_rfc5322_fields() {
        assert_eq!(
            rfc5322_fields(&HashMap::from([
                ("Subject".into(), "blah".into()),
                ("To".into(), "foo@bar.com".into()),
            ])),
            "Subject: blah\r\nTo: foo@bar.com",
        )
    }

    #[test]
    fn rfc5322_body_no_linebreak() {
        let body = ".".repeat(78);
        assert_eq!(rfc5322_body(&body), ".".repeat(78))
    }

    #[test]
    fn rfc5322_body_force_linebreak() {
        let body = ".".repeat(80);
        assert_eq!(
            rfc5322_body(&body),
            format!("{}\r\n{}", ".".repeat(78), ".".repeat(2)),
        )
    }

    #[test]
    fn rfc5322_body_with_extant_linebreaks() {
        let body = "hi there, yes, look:\r\n```\r\nfoo\r\n```\r\n\r\nMany thanks,";
        assert_eq!(rfc5322_body(body), body)
    }

    #[test]
    fn rfc5322_body_with_extant_and_new_linebreaks() {
        let body = format!(
            "hi there, yes, look:\r\n```\r\nfo{}\r\n```\r\n\r\nMany thanks,",
            "o".repeat(200)
        );
        assert_eq!(
            rfc5322_body(&body),
            format!(
                "hi there, yes, look:\r\n```\r\nfo{}\r\n{}\r\n{}\r\n```\r\n\r\nMany thanks,",
                "o".repeat(76),
                "o".repeat(78),
                "o".repeat(46),
            )
        )
    }

    #[test]
    fn rfc5322_fields_bcc_blind() {
        assert_eq!(
            rfc5322_fields(&HashMap::from([
                ("Bcc".into(), "foo@bar.com".into()),
                ("To".into(), "bar@foo.com".into()),
            ])),
            "Bcc:\r\nTo: bar@foo.com",
        )
    }
}

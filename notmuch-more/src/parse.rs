use std::convert::TryFrom;

use anyhow::anyhow;
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
    println!("Opening id:{id}");
    let msg = db
        .find_message(&id)?
        .ok_or_else(|| anyhow!("Message {} not found", id))?;
    let contents = &std::fs::read(msg.filename())?;

    println!("Parsing id:{id}");
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

#[cfg(test)]
mod tests {
    use super::*;
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
}

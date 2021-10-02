use anyhow::anyhow;
use notmuch::Database;

use crate::NotmuchMoreError;

mod addresses;
mod body;
mod error;
mod headers;

pub use addresses::EmlAddr;
pub use addresses::Mailbox;
pub use body::EmlBody;
pub use error::EmlParseError;
pub use headers::EmlMeta;

pub fn parse_eml(db: &Database, id: String) -> Result<EmlBody, NotmuchMoreError> {
    println!("Opening id:{}", id);
    let msg = db
        .find_message(&id)?
        .ok_or_else(|| anyhow!("Message {} not found", id))?;
    let contents = &std::fs::read(msg.filename())?;

    println!("Parsing id:{}", id);
    body::parse_body_part(&mailparse::parse_mail(contents)?)
}

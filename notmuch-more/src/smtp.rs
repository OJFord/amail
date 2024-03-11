use std::io::Write;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;
use lettre::transport::smtp;
use lettre::Transport;
use notmuch::Database;
use notmuch::Error::NotmuchError;
use tempfile::NamedTempFile;

use crate::error::NotmuchMoreError;

pub struct Smtp {
    transport: smtp::SmtpTransport,
}

impl Smtp {
    pub fn new(host: String, user: String, password: String) -> Self {
        Self {
            transport: smtp::SmtpTransport::relay(&host)
                .expect("Failed to create SMTP client")
                .credentials(smtp::authentication::Credentials::new(user, password))
                .build(),
        }
    }

    pub fn send(
        &self,
        db: &Database,
        to: Vec<String>,
        from: String,
        eml: String,
    ) -> Result<(), NotmuchMoreError> {
        let envelope = lettre::address::Envelope::new(
            Some(lettre::Address::from_str(&from)?),
            to.iter()
                .map(|e| lettre::Address::from_str(e))
                .collect::<Result<_, _>>()?,
        )?;

        let response = self.transport.send_raw(&envelope, eml.as_ref())?;
        match response.is_positive() {
            true => {
                println!("[INFO] Message sent");
                let mut file = NamedTempFile::new_in(format!("{}/sent/", db.path().display()))?;
                write!(file, "{eml}")?;
                println!("[TRACE] Sent message written to {}", file.path().display());

                let index_opts = db.default_indexopts()?;
                let message = db.index_file(&file.path().as_os_str(), None)?;
                println!("[TRACE] Sent message indexed as {}", message.id());

                message.add_tag("sent")?;

                let path = file.path().with_file_name::<String>(message.id().into());
                match file.persist_noclobber(&path) {
                    Ok(_) => {
                        println!("[TRACE] Renamed message {}, reindexing", message.id());
                        match db.index_file(&path, None) {
                            Err(NotmuchError(notmuch::Status::DuplicateMessageID)) => {
                                println!("[TRACE] New path indexed");
                                message.reindex(index_opts)?;
                                println!("[TRACE] Message reindexed");
                                Ok(())
                            }
                            _ => Err(NotmuchMoreError::Other(anyhow!(
                                "Error renaming sent message"
                            ))),
                        }
                    }
                    Err(e) => match e.file.keep() {
                        Ok((_, pathbuf)) => Err(NotmuchMoreError::Other(anyhow!(
                            "Failed to persist sent mail, kept at {}",
                            pathbuf.display()
                        ))),
                        Err(_) => Err(NotmuchMoreError::Other(anyhow!(
                            "Failed to persist or keep sent mail: {}",
                            eml
                        ))),
                    },
                }
            }
            false => Err(anyhow!(
                "SMTP error {}: {}",
                response.code(),
                response.message().join("\n")
            )
            .into()),
        }
    }
}

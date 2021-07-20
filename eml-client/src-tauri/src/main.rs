#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::process::Command;

use anyhow::anyhow;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum AmailError {
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    NotMuchError(#[from] notmuch::Error),
    #[error(transparent)]
    ParseError(#[from] mailparse::MailParseError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<AmailError> for tauri::InvokeError {
    fn from(e: AmailError) -> tauri::InvokeError {
        Self::from(format!("{}", e))
    }
}

#[derive(Serialize)]
struct EmlMeta {
    author: String,
    subject: String,
    timestamp: i64,
}

#[tauri::command]
fn list_eml() -> Result<Vec<(EmlMeta, String)>, AmailError> {
    let mut db_path = String::from_utf8(
        Command::new("notmuch")
            .args(&["config", "get", "database.path"])
            .output()
            .expect("Failed to find notmuch database.path")
            .stdout,
    )
    .expect("Non-UTF8 database.path");
    db_path = db_path.trim().to_string();

    let db = notmuch::Database::open(&db_path, notmuch::DatabaseMode::ReadOnly)?;
    let eml_query = db.create_query("tag:inbox")?;
    eml_query.set_sort(notmuch::Sort::NewestFirst);
    let emls = eml_query.search_messages()?;

    emls.into_iter()
        .take(25)
        .map(|eml| {
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
                                mailparse::MailAddr::Group(g) => g.group_name.to_owned(),
                                mailparse::MailAddr::Single(s) => s
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
            .and_then(|meta| {
                Ok((
                    meta,
                    String::from(
                        eml.filename()
                            .to_str()
                            .ok_or_else(|| anyhow!("Non-unicode path"))?,
                    ),
                ))
            })
        })
        .collect()
}

#[tauri::command]
fn view_eml(id: String) -> Result<String, AmailError> {
    let contents = &fs::read(id)?;
    let eml = mailparse::parse_mail(contents)?;

    if eml.ctype.mimetype == "text/html" || eml.ctype.mimetype == "text/plain" {
        return Ok(eml.get_body()?);
    }

    for part in eml.subparts {
        if part.ctype.mimetype == "text/html" || part.ctype.mimetype == "text/plain" {
            return Ok(part.get_body()?);
        }
    }

    Err(AmailError::Other(anyhow!("No plaintext version")))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_eml, view_eml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

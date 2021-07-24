#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::convert::TryFrom;
use std::fs;
use std::process::Command;

use anyhow::anyhow;
use thiserror::Error;

mod eml;
use eml::EmlMeta;
use eml::EmlParseError;

#[derive(Debug, Error)]
pub enum AmailError {
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
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

struct State {
    db_path: String,
}

impl State {
    fn open_db_ro(&self) -> Result<notmuch::Database, AmailError> {
        Ok(notmuch::Database::open(
            &self.db_path,
            notmuch::DatabaseMode::ReadOnly,
        )?)
    }
}

#[tauri::command]
fn list_eml(state: tauri::State<State>) -> Result<Vec<Result<EmlMeta, EmlParseError>>, AmailError> {
    let db = state.open_db_ro()?;
    let eml_query = db.create_query("tag:inbox")?;
    eml_query.set_sort(notmuch::Sort::NewestFirst);
    let emls = eml_query.search_messages()?;

    emls.into_iter()
        .take(25)
        .map(|m| Ok(EmlMeta::try_from(&m)))
        .collect()
}

#[tauri::command]
fn view_eml(state: tauri::State<State>, eml_meta: EmlMeta) -> Result<String, AmailError> {
    let db = state.open_db_ro()?;
    let msg = db
        .find_message(&eml_meta.id)?
        .ok_or_else(|| anyhow!("Message {} not found", eml_meta.id))?;
    let contents = &fs::read(msg.filename())?;
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
    let mut db_path = String::from_utf8(
        Command::new("notmuch")
            .args(&["config", "get", "database.path"])
            .output()
            .expect("Failed to find notmuch database.path")
            .stdout,
    )
    .expect("Non-UTF8 database.path");
    db_path = db_path.trim().to_string();

    tauri::Builder::default()
        .manage(State { db_path })
        .invoke_handler(tauri::generate_handler![list_eml, view_eml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

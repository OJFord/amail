#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::convert::TryFrom;
use std::fs;
use std::process::Command;

use anyhow::anyhow;

mod eml;
mod error;
use eml::EmlMeta;
use error::AmailError;
use error::EmlParseError;

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

    fn open_db_rw(&self) -> Result<notmuch::Database, AmailError> {
        Ok(notmuch::Database::open(
            &self.db_path,
            notmuch::DatabaseMode::ReadWrite,
        )?)
    }
}

#[tauri::command]
fn apply_tag(state: tauri::State<State>, query: String, tag: String) -> Result<(), AmailError> {
    println!("Adding tag:{} where {}", tag, query);

    let db = state.open_db_rw()?;
    let eml_query = db.create_query(&format!("({}) and not tag:{}", query, tag))?;

    for eml in eml_query.search_messages()? {
        eml.add_tag(&tag)?;
    }
    Ok(())
}

#[tauri::command]
fn rm_tag(state: tauri::State<State>, query: String, tag: String) -> Result<(), AmailError> {
    println!("Removing tag:{} where {}", tag, query);

    let db = state.open_db_rw()?;
    let eml_query = db.create_query(&format!("({}) and tag:{}", query, tag))?;

    for eml in eml_query.search_messages()? {
        eml.remove_tag(&tag)?;
    }
    Ok(())
}

#[tauri::command]
fn count_matches(state: tauri::State<State>, query: String) -> Result<u32, AmailError> {
    println!("Counting matches for query: {}", query);

    let db = state.open_db_ro()?;
    let eml_query = db.create_query(&query)?;

    eml_query.count_messages().map_err(AmailError::from)
}

#[tauri::command]
fn list_eml(
    state: tauri::State<State>,
    query: String,
) -> Result<Vec<Result<EmlMeta, EmlParseError>>, AmailError> {
    println!("Executing query: {}", query);

    let db = state.open_db_ro()?;
    let eml_query = db.create_query(&query)?;
    eml_query.set_sort(notmuch::Sort::NewestFirst);
    let emls = eml_query.search_messages()?;

    emls.into_iter()
        .take(25)
        .map(|m| Ok(EmlMeta::try_from(&m)))
        .collect()
}

#[tauri::command]
fn list_tags(state: tauri::State<State>) -> Result<Vec<String>, AmailError> {
    println!("Listing tags");

    let db = state.open_db_ro()?;
    db.all_tags()
        .map(|ts| ts.into_iter().collect::<Vec<String>>())
        .map_err(AmailError::from)
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
        .invoke_handler(tauri::generate_handler![
            apply_tag,
            count_matches,
            list_eml,
            list_tags,
            rm_tag,
            view_eml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

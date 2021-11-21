#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::env;
use std::process::Command;

use anyhow::anyhow;
use notmuch_more::compose;
use notmuch_more::parse;
use notmuch_more::parse::EmlBody;
use notmuch_more::parse::EmlMeta;
use notmuch_more::parse::EmlParseError;
use notmuch_more::query;
use notmuch_more::smtp;
use notmuch_more::tags;
use notmuch_more::Database;

mod error;
use self::error::AmailError;

struct State {
    db: Database,
    smtp: smtp::Smtp,
}

#[tauri::command]
fn apply_tag(state: tauri::State<State>, query: String, tag: String) -> Result<(), AmailError> {
    let db = state.db.open_rw()?;
    Ok(tags::apply_tag(&db, query, tag)?)
}

#[tauri::command]
fn rm_tag(state: tauri::State<State>, query: String, tag: String) -> Result<(), AmailError> {
    let db = state.db.open_rw()?;
    Ok(tags::rm_tag(&db, query, tag)?)
}

#[tauri::command]
fn count_matches(state: tauri::State<State>, query: String) -> Result<u32, AmailError> {
    let db = state.db.open_ro()?;
    Ok(query::count_matches(&db, query)?)
}

#[tauri::command]
fn list_eml(
    state: tauri::State<State>,
    query: String,
) -> Result<Vec<Result<EmlMeta, EmlParseError>>, AmailError> {
    let db = state.db.open_ro()?;
    Ok(query::list_eml(&db, query)?)
}

#[tauri::command]
fn list_tags(state: tauri::State<State>) -> Result<Vec<String>, AmailError> {
    let db = state.db.open_ro()?;
    Ok(tags::list_tags(&db)?)
}

#[tauri::command]
fn view_eml(state: tauri::State<State>, id: String) -> Result<EmlBody, AmailError> {
    let db = state.db.open_ro()?;
    Ok(parse::parse_eml(&db, id)?.1)
}

#[tauri::command]
fn get_name() -> String {
    println!("Getting user's name");
    whoami::realname()
}

#[tauri::command]
fn send_eml(
    state: tauri::State<State>,
    headers: HashMap<String, String>,
    body: String,
) -> Result<(), AmailError> {
    let db = state.db.open_rw()?;
    Ok(state.smtp.send(
        &db,
        headers
            .get("To")
            .ok_or(anyhow!("Missing To header"))?
            .split(",")
            .map(String::from)
            .collect(),
        headers
            .get("From")
            .ok_or(anyhow!("Missing From header"))?
            .into(),
        compose::rfc5322_message(&headers, &body),
    )?)
}

#[tauri::command]
fn get_reply_template(
    state: tauri::State<State>,
    id: String,
) -> Result<compose::ReplyTemplate, AmailError> {
    let db = state.db.open_rw()?;
    Ok(compose::template_reply(&db, id)?)
}

#[tauri::command]
fn preview_eml(
    _: tauri::State<State>,
    headers: HashMap<String, String>,
    body: String,
) -> Result<String, AmailError> {
    Ok(compose::rfc5322_message(&headers, &body))
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
    let db = Database::new(db_path);

    let smtp = smtp::Smtp::new(
        env::var("SMTP_HOST").expect("Missing $SMTP_HOST"),
        env::var("SMTP_USER").expect("Missing $SMTP_USER"),
        env::var("SMTP_PASS").expect("Missing $SMTP_PASS"),
    );

    tauri::Builder::default()
        .manage(State { db, smtp })
        .invoke_handler(tauri::generate_handler![
            apply_tag,
            count_matches,
            get_name,
            get_reply_template,
            list_eml,
            list_tags,
            preview_eml,
            rm_tag,
            send_eml,
            view_eml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

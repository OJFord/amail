#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::convert::TryFrom;
use std::fs;
use std::process::Command;

use anyhow::anyhow;
use email::mimeheaders::MimeContentType;
use email::MimeMultipartType;
use itertools::Itertools;
use serde::Serialize;

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

#[derive(Clone, Debug, Default, Serialize)]
pub struct EmlBody {
    pub alternatives: Vec<EmlBody>,
    pub content: String,
    pub content_encoded: Option<Vec<u8>>,
    pub disposition: String,
    pub extra: Vec<EmlBody>,
    pub filename: Option<String>,
    pub is_cleaned_html: bool,
    pub mimetype: String,
    pub signature: Option<Box<EmlBody>>,
    pub size: Option<String>,
}

fn parse_body_part(part: &mailparse::ParsedMail) -> Result<EmlBody, AmailError> {
    let mimect: MimeContentType = part
        .ctype
        .mimetype
        .split_once('/')
        .map(|(s1, s2)| (s1.into(), s2.into()))
        .ok_or_else(|| anyhow!("Failed to parse mimetype: {}", part.ctype.mimetype))?;

    let content_disp = part.get_content_disposition();

    match MimeMultipartType::from_content_type(mimect) {
        None => match part.ctype.mimetype.as_str() {
            "text/html" => Ok(EmlBody {
                content: ammonia::Builder::default()
                    .rm_tag_attributes("img", &["src"])
                    .clean(&part.get_body()?)
                    .to_string(),
                disposition: format!("{:?}", content_disp.disposition),
                filename: content_disp.params.get("filename").map(|f| f.into()),
                is_cleaned_html: true,
                mimetype: part.ctype.mimetype.to_owned(),
                size: content_disp.params.get("size").map(|f| f.into()),
                ..Default::default()
            }),
            _ => Ok(EmlBody {
                content: part.get_body()?,
                content_encoded: Some(part.get_body_raw()?),
                disposition: format!("{:?}", content_disp.disposition),
                filename: content_disp.params.get("filename").map(|f| f.into()),
                mimetype: part.ctype.mimetype.to_owned(),
                size: content_disp.params.get("size").map(|f| f.into()),
                ..Default::default()
            }),
        },

        Some(MimeMultipartType::Alternative) => {
            let mut first = parse_body_part(&part.subparts[0])?;
            first.alternatives = part.subparts[1..]
                .iter()
                .map(parse_body_part)
                .collect::<Result<_, _>>()?;
            Ok(first)
        }

        Some(MimeMultipartType::Mixed) => {
            let mut first = parse_body_part(&part.subparts[0])?;
            first.extra = part.subparts[1..]
                .iter()
                .map(parse_body_part)
                .collect::<Result<_, _>>()?;

            Ok(first)
        }

        Some(MimeMultipartType::Signed) => {
            let mut first = parse_body_part(&part.subparts[0])?;
            first.signature = Some(Box::new(parse_body_part(
                part.subparts[1..]
                    .iter()
                    .exactly_one()
                    .map_err(|_| anyhow!("Expected exactly one signature for signed part"))?,
            )?));

            Ok(first)
        }

        Some(t) => Err(anyhow!("Not implemented: {:?}", t).into()),
    }
}

#[tauri::command]
fn view_eml(state: tauri::State<State>, id: String) -> Result<EmlBody, AmailError> {
    println!("Opening id:{}", id);
    let db = state.open_db_ro()?;
    let msg = db
        .find_message(&id)?
        .ok_or_else(|| anyhow!("Message {} not found", id))?;
    let contents = &fs::read(msg.filename())?;

    println!("Parsing id:{}", id);
    parse_body_part(&mailparse::parse_mail(contents)?)
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

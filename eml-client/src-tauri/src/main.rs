#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

use anyhow::anyhow;
use etcetera::base_strategy;
use etcetera::base_strategy::BaseStrategy;
use mailparse::MailHeaderMap;
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
    ParseError(#[from] mailparse::MailParseError),
    #[error(transparent)]
    HomeDirError(#[from] etcetera::HomeDirError),
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
    let mut emls = fs::read_dir(
        base_strategy::choose_base_strategy()?
            .data_dir()
            .join("amail")
            .join("eml"),
    )?
    .map(|e| e.ok())
    .filter(|e| e.is_some())
    .flatten()
    .collect::<Vec<_>>();

    emls.sort_by_key(|e| std::cmp::Reverse(e.metadata().and_then(|m| m.modified()).unwrap()));

    emls.iter()
        .take(25)
        .map(|eml| {
            Ok(mailparse::parse_headers(&fs::read(eml.path())?)?.0)
                .and_then(|eml| {
                    Ok(EmlMeta {
                        author: eml
                            .get_first_header("From")
                            .ok_or_else(|| anyhow!("Missing from"))
                            .and_then(|ref f| Ok(mailparse::addrparse_header(f)?))
                            .and_then(|a| {
                                a.extract_single_info()
                                    .ok_or_else(|| anyhow!("Expected single from address"))
                                    .map(|s| s.display_name.unwrap_or(s.addr))
                            })?,

                        timestamp: eml
                            .get_first_value("Date")
                            .ok_or_else(|| anyhow!("Missing date"))
                            .and_then(|ref d| Ok(mailparse::dateparse(d)?))?,

                        subject: eml
                            .get_first_value("Subject")
                            .ok_or_else(|| anyhow!("Missing subject"))?,
                    })
                })
                .and_then(|meta| {
                    Ok((
                        meta,
                        String::from(
                            eml.path()
                                .to_str()
                                .ok_or_else(|| anyhow!("Non-unicode  path"))?,
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

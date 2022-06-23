use anyhow::anyhow;
use email::mimeheaders::MimeContentType;
use email::MimeMultipartType;
use itertools::Itertools;
use regex::Regex;
use serde::Serialize;

use crate::NotmuchMoreError;

#[derive(Clone, Debug, Default, Serialize)]
pub struct EmlBody {
    pub alternatives: Vec<EmlBody>,
    pub content: String,
    pub content_base64: Option<String>,
    pub content_encoded: Option<Vec<u8>>,
    pub disposition: String,
    pub extra: Vec<EmlBody>,
    pub filename: Option<String>,
    pub is_cleaned_html: bool,
    pub mimetype: String,
    pub signature: Option<Box<EmlBody>>,
    pub size: Option<String>,
}

pub(crate) fn parse_body_part(part: &mailparse::ParsedMail) -> Result<EmlBody, NotmuchMoreError> {
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
                content: {
                    let b = ammonia::Builder::default()
                        .set_tag_attribute_value("a", "target", "_blank")
                        .rm_tag_attributes("img", &["src"])
                        .clean(&part.get_body()?);

                    Regex::new("href=\"([^\"]+)")
                        .unwrap()
                        .replace_all(&b.to_string(), "href=\"$1\" title=\"$1\"")
                        .into()
                },
                content_base64: match part.get_body_encoded() {
                    mailparse::body::Body::Base64(body) => {
                        String::from_utf8(body.get_raw().into()).map_or_else(|_| None, Some)
                    }
                    _ => None,
                },
                content_encoded: Some(part.get_body_raw()?),
                disposition: format!("{:?}", content_disp.disposition),
                filename: content_disp.params.get("filename").map(|f| f.into()),
                is_cleaned_html: true,
                mimetype: part.ctype.mimetype.to_owned(),
                size: content_disp.params.get("size").map(|f| f.into()),
                ..Default::default()
            }),
            _ => Ok(EmlBody {
                content: part.get_body()?,
                content_base64: match part.get_body_encoded() {
                    mailparse::body::Body::Base64(body) => {
                        String::from_utf8(body.get_raw().into()).map_or_else(|_| None, Some)
                    }
                    _ => None,
                },
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

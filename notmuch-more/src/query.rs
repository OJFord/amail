use std::convert::TryFrom;

use notmuch::Database;

use crate::error::NotmuchMoreError;
use crate::parse::EmlMeta;
use crate::parse::EmlParseError;

pub fn count_matches(db: &Database, query: String) -> Result<u32, NotmuchMoreError> {
    println!("Counting matches for query: {}", query);
    let eml_query = db.create_query(&query)?;
    eml_query.count_messages().map_err(NotmuchMoreError::from)
}

pub fn list_eml(
    db: &Database,
    query: String,
) -> Result<Vec<Result<EmlMeta, EmlParseError>>, NotmuchMoreError> {
    println!("Executing query: {}", query);

    let eml_query = db.create_query(&query)?;
    eml_query.set_sort(notmuch::Sort::NewestFirst);
    let emls = eml_query.search_messages()?;

    emls.into_iter()
        .take(25)
        .map(|m| Ok(EmlMeta::try_from(&m)))
        .collect()
}

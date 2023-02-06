use notmuch::Database;

use crate::error::NotmuchMoreError;

pub fn apply_tag(db: &Database, query: String, tag: String) -> Result<(), NotmuchMoreError> {
    println!("Adding tag:{tag} where {query}");
    let eml_query = db.create_query(&format!("({query}) and not tag:{tag}"))?;
    for eml in eml_query.search_messages()? {
        eml.add_tag(&tag)?;
    }
    Ok(())
}

pub fn rm_tag(db: &Database, query: String, tag: String) -> Result<(), NotmuchMoreError> {
    println!("Removing tag:{tag} where {query}");
    let eml_query = db.create_query(&format!("({query}) and tag:{tag}"))?;
    for eml in eml_query.search_messages()? {
        eml.remove_tag(&tag)?;
    }
    Ok(())
}

pub fn list_tags(db: &Database) -> Result<Vec<String>, NotmuchMoreError> {
    println!("Listing tags");
    db.all_tags()
        .map(|ts| ts.into_iter().collect::<Vec<String>>())
        .map_err(NotmuchMoreError::from)
}

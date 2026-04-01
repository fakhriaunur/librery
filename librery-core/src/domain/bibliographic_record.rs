use super::ids::BibRecordId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibliographicRecord {
    pub id: BibRecordId,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub marc21: Option<String>,
}

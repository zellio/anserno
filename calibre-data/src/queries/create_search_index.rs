use crate::queries::StaticQuery;

pub struct CreateSearchIndex;

/// Create an fts5 virtual table for search functionality.
impl StaticQuery for CreateSearchIndex {
    const QUERY: &str = indoc::indoc! {r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS "anserno_search_index" USING fts5 (
            "title", "sort", "authors", "series", "formats", "description"
        );
    "#};
}

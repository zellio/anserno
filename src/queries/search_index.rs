use crate::queries::AnsernoQuery;

pub struct SearchIndex {}

impl AnsernoQuery for SearchIndex {
    fn create_query_str() -> &'static str {
        r#"
CREATE VIRTUAL TABLE IF NOT EXISTS "anserno_search_index" USING fts5 (
  "title", "sort", "authors", "series", "formats", "description"
)
"#
    }

    fn populate_query_str() -> &'static str {
        r#"
INSERT INTO "anserno_search_index" (
  "rowid", "title", "sort", "authors", "series", "formats", "description"
)
SELECT
    "books"."id" AS "rowid",
    "books"."title" AS "title",
    "books"."sort" AS "sort",
    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "authors"."name" || '@'), '@,', ', '), '@') AS "authors",
    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "series"."name" || '@'), '@,', ', '), '@') AS "series",
    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "data"."format" || '@'), '@,', ', '), '@') AS "formats",
    "comments"."text" AS "description"
FROM
    "books"
    LEFT JOIN "data" ON "books"."id" = "data"."book"
    LEFT JOIN "comments" ON "books"."id" = "comments"."book"
    LEFT JOIN "books_authors_link" ON "books"."id" = "books_authors_link"."book"
    LEFT JOIN "authors" ON "authors"."id" = "books_authors_link"."author"
    LEFT JOIN "books_series_link" ON "books"."id" = "books_series_link"."book"
    LEFT JOIN "series" ON "series"."id" = "books_series_link"."series"
GROUP BY
    "books"."id"
"#
    }
}

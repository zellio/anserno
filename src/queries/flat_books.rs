use crate::queries::AnsernoQuery;

pub struct FlatBooks {}

impl AnsernoQuery for FlatBooks {
    fn create_query_str() -> &'static str {
        r#"
CREATE TABLE IF NOT EXISTS anserno_flat_books (
    "id" INTEGER PRIMARY KEY,
    "title" TEXT,
    "sort" TEXT,
    "path" TEXT,
    "authors" TEXT,
    "series" TEXT,
    "series_index" REAL,
    "formats" TEXT,
    "description" TEXT,
    FOREIGN KEY(id) REFERENCES books(id)
);
"#
    }

    fn populate_query_str() -> &'static str {
        r#"
INSERT INTO anserno_flat_books (
   "id", "title", "sort", "path", "authors", "series", "series_index", "formats", "description"
)
SELECT
    "books"."id" AS "id",
    "books"."title" AS "title",
    "books"."sort" AS "sort",
    "books"."path" AS "path",
    COALESCE("author_data"."data", json('{}')) AS "authors",
    COALESCE("series_data"."data", json('{}')) AS "series",
    "books"."series_index" AS "series_index",
    COALESCE("format_data"."data", json('{}')) AS "formats",
    COALESCE("comments"."text", '') AS "description"
FROM
    books
LEFT JOIN (
    SELECT
        books.id AS book_id,
        json_group_object(authors.id, authors.name) AS data
    FROM
        authors
    LEFT JOIN books_authors_link
        ON authors.id = books_authors_link.author
    LEFT JOIN books
        ON books.id = books_authors_link.book
    GROUP BY
        books.id
) author_data ON books.id = author_data.book_id
LEFT JOIN (
    SELECT
        books.id as book_id,
        json_group_object(series.id, series.name) AS data
    FROM
        series
    LEFT JOIN books_series_link
        ON series.id = books_series_link.series
    LEFT JOIN books
        ON books.id = books_series_link.book
    GROUP BY
        books.id
) series_data ON books.id = series_data.book_id
LEFT JOIN (
    SELECT
        books.id AS book_id,
        json_group_object(data.format, data.name) AS data
    FROM
        data
    LEFT JOIN books
        ON books.id = data.book
    GROUP BY
        books.id
) format_data ON books.id = format_data.book_id
LEFT JOIN comments ON books.id = comments.book
"#
    }
}

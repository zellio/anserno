use crate::queries::StaticQuery;

pub struct CreateFlatBooksView;

/// Create the flat_books view.
///
/// flat_books is a denomalized representation of books and supporting data to
/// facilitate display to the user. It collects the `authors`, `series`, and
/// `format` data into json maps, along with a few other fields as their
/// string values.
impl StaticQuery for CreateFlatBooksView {
    const QUERY: &str = indoc::indoc! {r#"
        CREATE VIEW IF NOT EXISTS anserno_flat_books (
           "id", "title", "sort", "path", "authors", "series", "series_index", "formats", "description"
        ) AS
        WITH
            author_json AS (
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
            ),
            series_json AS (
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
            ),
            format_json AS (
                SELECT
                    books.id AS book_id,
                    json_group_object(data.format, data.name) AS data
                FROM
                    data
                LEFT JOIN books
                    ON books.id = data.book
                GROUP BY
                    books.id
            )
        SELECT
            "books"."id" AS "id",
            "books"."title" AS "title",
            "books"."sort" AS "sort",
            "books"."path" AS "path",
            COALESCE("author_json"."data", json('{}')) AS "authors",
            COALESCE("series_json"."data", json('{}')) AS "series",
            "books"."series_index" AS "series_index",
            COALESCE("format_json"."data", json('{}')) AS "formats",
            COALESCE("comments"."text", '') AS "description"
        FROM
            books
        LEFT JOIN
            author_json ON books.id = author_json.book_id
        LEFT JOIN
            series_json ON books.id = series_json.book_id
        LEFT JOIN
            format_json ON books.id = format_json.book_id
        LEFT JOIN
            comments ON books.id = comments.book
    "#};
}

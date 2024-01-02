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

// use sea_orm::{ColumnTrait, EntityTrait, JoinType, QuerySelect, RelationTrait};
// use serde::{Deserialize, Serialize};

// use crate::entities::{
//         authors, books, books_authors_link, books_series_link, comments, data, series,
// };

// use crate::queries::function_name::FunctionName;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Model {
//         pub id: i32,
//         pub title: String,
//         pub sort: String,
//         pub description: String,
//         pub path: String,
//         pub authors: serde_json::Value,
//         pub series: serde_json::Value,
//         pub series_index: f64,
//         pub formats: serde_json::Value,
// }

// pub fn query() -> sea_orm::Select<books::Entity> {
//         books::Entity::find()
//                 .select_only()
//                 .column(books::Column::Id)
//                 .column(books::Column::Title)
//                 .column(books::Column::Sort)
//                 .column_as(comments::Column::Text.if_null(""), "description")
//                 .column(books::Column::Path)
//                 .column_as(
//                         FunctionName("json_patch").as_function_with_args([
//                                 "{}".into(),
//                                 FunctionName("json_group_object").as_function_with_args([
//                                         authors::Column::Id.if_null(0),
//                                         authors::Column::Name.into_expr().into(),
//                                 ]),
//                         ]),
//                         "authors",
//                 )
//                 .column_as(
//                         FunctionName("json_patch").as_function_with_args([
//                                 "{}".into(),
//                                 FunctionName("json_group_object").as_function_with_args([
//                                         series::Column::Id.if_null(0),
//                                         series::Column::Name.into_expr().into(),
//                                 ]),
//                         ]),
//                         "series",
//                 )
//                 .column(books::Column::SeriesIndex)
//                 .column_as(
//                         FunctionName("json_patch").as_function_with_args([
//                                 "{}".into(),
//                                 FunctionName("json_group_object").as_function_with_args([
//                                         data::Column::Format.into_expr().into(),
//                                         data::Column::Name.into_expr().into(),
//                                 ]),
//                         ]),
//                         "formats",
//                 )
//                 .left_join(data::Entity)
//                 .left_join(comments::Entity)
//                 .left_join(books_authors_link::Entity)
//                 .join_rev(
//                         JoinType::LeftJoin,
//                         authors::Relation::BooksAuthorsLink.def(),
//                 )
//                 .left_join(books_series_link::Entity)
//                 .join_rev(JoinType::LeftJoin, series::Relation::BooksSeriesLink.def())
//                 .group_by(books::Column::Id)
// }

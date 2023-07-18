use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QuerySelect, RelationTrait,
};
use serde::Serialize;

use crate::{
    entities::*,
    error::{AnsernoError, AnsernoResult},
    queries::FunctionName,
};

#[derive(Debug, Clone, FromQueryResult, Serialize)]
pub struct FlatBook {
    pub id: i32,
    pub title: String,
    pub sort: String,
    pub description: String,
    pub path: String,
    pub authors: serde_json::Value,
    pub series: serde_json::Value,
    pub series_index: f64,
    pub formats: serde_json::Value,
}

pub fn flat_books_query() -> sea_orm::Select<books::Entity> {
    books::Entity::find()
        .select_only()
        .column(books::Column::Id)
        .column(books::Column::Title)
        .column(books::Column::Sort)
        .column_as(comments::Column::Text.if_null(""), "description")
        .column(books::Column::Path)
        .column_as(
            FunctionName("json_patch").as_function_with_args([
                "{}".into(),
                FunctionName("json_group_object").as_function_with_args([
                    authors::Column::Id.if_null(0),
                    authors::Column::Name.into_expr().into(),
                ]),
            ]),
            "authors",
        )
        .column_as(
            FunctionName("json_patch").as_function_with_args([
                "{}".into(),
                FunctionName("json_group_object").as_function_with_args([
                    series::Column::Id.if_null(0),
                    series::Column::Name.into_expr().into(),
                ]),
            ]),
            "series",
        )
        .column(books::Column::SeriesIndex)
        .column_as(
            FunctionName("json_patch").as_function_with_args([
                "{}".into(),
                FunctionName("json_group_object").as_function_with_args([
                    data::Column::Format.into_expr().into(),
                    data::Column::Name.into_expr().into(),
                ]),
            ]),
            "formats",
        )
        .left_join(data::Entity)
        .left_join(comments::Entity)
        .left_join(books_authors_link::Entity)
        .join_rev(
            JoinType::LeftJoin,
            authors::Relation::BooksAuthorsLink.def(),
        )
        .left_join(books_series_link::Entity)
        .join_rev(JoinType::LeftJoin, series::Relation::BooksSeriesLink.def())
        .group_by(books::Column::Id)
}

pub async fn get_flat_books_by_id(
    conn: &DatabaseConnection,
    ids: Vec<i32>,
) -> AnsernoResult<Vec<FlatBook>> {
    Ok(flat_books_query()
        .filter(books::Column::Id.is_in(ids))
        .into_model::<FlatBook>()
        .all(conn)
        .await?)
}

pub async fn get_flat_book_by_id(conn: &DatabaseConnection, id: i32) -> AnsernoResult<FlatBook> {
    flat_books_query()
        .filter(books::Column::Id.eq(id))
        .into_model::<FlatBook>()
        .one(conn)
        .await?
        .ok_or(AnsernoError::NotFound("No FlatBook(id={id}".to_string()))
}

use actix_web::{web, Responder};
use sea_orm::{EntityTrait, PaginatorTrait, PrimaryKeyToColumn};
use serde::Serialize;

use crate::{
    app,
    entities::*,
    error::{AnsernoError, AnsernoWebResult},
    jhal::{JhalLinkBuilder, JhalModel, JhalResource, JhalResourceBuilder},
    pagination::{PaginationUrlQueryParams, Paginator, SlicePaginator, SlicePaginatorBuilder},
};

pub async fn get_index(ctx: web::Data<app::Context>) -> AnsernoWebResult<impl Responder> {
    JhalResourceBuilder::default()
        .link((
            "authors".to_string(),
            vec![
                JhalLinkBuilder::default()
                    .href("/authors")
                    .title("authors")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                JhalLinkBuilder::default()
                    .href("/authors/{id}")
                    .templated(true)
                    .title("author")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
            ],
        ))
        .link((
            "books".to_string(),
            vec![
                JhalLinkBuilder::default()
                    .href("/books")
                    .title("books")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                JhalLinkBuilder::default()
                    .href("/books/{id}")
                    .templated(true)
                    .title("book")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
            ],
        ))
        .link((
            "series".to_string(),
            vec![
                JhalLinkBuilder::default()
                    .href("/series")
                    .title("series")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                JhalLinkBuilder::default()
                    .href("/series/{id}")
                    .templated(true)
                    .title("series")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
            ],
        ))
        .build()
        .map(web::Json)
        .map_err(|err| err.with_context(&ctx))
}

pub async fn get<E>(
    ctx: web::Data<app::Context>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder>
where
    E: EntityTrait,
    <E as sea_orm::EntityTrait>::Model: JhalModel + Serialize + Sync,
    <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
{
    let conn = &ctx.database_connection;
    let model_count = E::find()
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(model_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    JhalResource::from_paginated_models::<E, SlicePaginator>(
        paginator
            .selector(E::find())
            .all(conn)
            .await
            .map_err(|err| AnsernoError::from(err).with_context(&ctx))?,
        paginator,
    )
    .map(web::Json)
    .map_err(|err| err.with_context(&ctx))
}

pub async fn get_author_by_id(
    ctx: web::Data<app::Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;
    let id = id.into_inner();

    let author = authors::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(AnsernoError::NotFound(format!("No Author(id={id})")))
        .map_err(|err| err.with_context(&ctx))?;

    JhalResource::from_author(conn, &author)
        .await
        .map(web::Json)
        .map_err(|err| err.with_context(&ctx))
}

pub async fn get_book_by_id(
    ctx: web::Data<app::Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;
    let id = id.into_inner();

    let book = books::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(AnsernoError::NotFound(format!("No Book(id={id})")).with_context(&ctx))?;

    JhalResource::from_book(conn, &book)
        .await
        .map(web::Json)
        .map_err(|err| err.with_context(&ctx))
}

pub async fn get_series_by_id(
    ctx: web::Data<app::Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;
    let id = id.into_inner();

    let series = series::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(AnsernoError::NotFound(format!("No Series(id={id})")).with_context(&ctx))?;

    JhalResource::from_series(conn, &series)
        .await
        .map(web::Json)
        .map_err(|err| err.with_context(&ctx))
}

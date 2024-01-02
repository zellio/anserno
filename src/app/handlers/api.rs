use crate::{
    app,
    error::{AnsernoError, AnsernoWebResult},
    hypertext_application_language::{LinkBuilder, Model, Resource, ResourceBuilder},
    pagination::{PaginationUrlQueryParams, Paginator, SlicePaginator, SlicePaginatorBuilder},
};
use actix_web::{web, Responder};
use sea_orm::{EntityTrait, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait};
use serde::Serialize;

pub async fn get_index(ctx: web::Data<app::Context>) -> AnsernoWebResult<impl Responder> {
    ResourceBuilder::default()
        .link((
            "authors".to_string(),
            vec![
                LinkBuilder::default()
                    .href("/authors")
                    .title("authors")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                LinkBuilder::default()
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
                LinkBuilder::default()
                    .href("/books")
                    .title("books")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                LinkBuilder::default()
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
                LinkBuilder::default()
                    .href("/series")
                    .title("series")
                    .build()
                    .map_err(|err| err.with_context(&ctx))?,
                LinkBuilder::default()
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
    <E as EntityTrait>::Model: Model + Serialize + Sync,
    <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
{
    let conn = &ctx.conn;
    let model_count = E::find()
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(model_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    Resource::from_paginated_models::<E, SlicePaginator>(
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

pub async fn get_by_id<E>(
    ctx: web::Data<app::Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: Model + Serialize + Sync,
    <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
    <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
{
    let conn = &ctx.conn;
    let id = id.into_inner();

    let model = E::find_by_id(id)
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(AnsernoError::NotFound(format!("Model found for {id}")))
        .map_err(|err| err.with_context(&ctx))?;

    model
        .as_resource(conn)
        .await
        .map(web::Json)
        .map_err(|err| err.with_context(&ctx))
}

use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{
    app::Context,
    entities::{books, books_series_link, series},
    error::{AnsernoError, AnsernoWebResult},
    pagination::{
        Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder,
        SubstringBucketPaginatorBuilder,
    },
    queries::{flat_books_query, get_flat_books_by_id, paginator::substring_buckets, FlatBook},
};

pub async fn get(
    ctx: web::Data<Context>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("title", "Series");
    tmpl_ctx.insert("url", "/series");
    tmpl_ctx.insert("include_jump", &true);

    let paginator = SubstringBucketPaginatorBuilder::from(pagination_query.into_inner())
        .bucket_column(series::Column::Sort)
        .buckets(
            substring_buckets::<series::Entity, series::Column, String>(series::Column::Sort, 1)
                .all(conn)
                .await
                .unwrap_or(vec![]),
        )
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let series = paginator
        .selector(series::Entity::find())
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    tmpl_ctx.insert("container", &series);

    let series_ids: Vec<i32> = series.iter().map(|series| series.id).collect();

    let series_books_ids = books_series_link::Entity::find()
        .filter(books_series_link::Column::Series.is_in(series_ids))
        .select_only()
        .column(books_series_link::Column::Book)
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let series_flat_books = get_flat_books_by_id(conn, series_books_ids)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut series_flat_books_map = HashMap::new();
    for (series_id, flat_book) in series_flat_books.into_iter().flat_map(|flat_series_book| {
        flat_series_book
            .series
            .as_object()
            .unwrap()
            .into_iter()
            .map(|(series_id, _)| {
                (
                    series_id.parse::<i32>().unwrap_or(0),
                    flat_series_book.clone(),
                )
            })
            .collect::<Vec<(i32, FlatBook)>>()
    }) {
        series_flat_books_map
            .entry(series_id)
            .or_insert_with(Vec::new)
            .push(flat_book);
    }

    for books in series_flat_books_map.values_mut() {
        books.sort_by(|a, b| a.series_index.partial_cmp(&b.series_index).unwrap())
    }

    tmpl_ctx.insert("paginator", &paginator);
    tmpl_ctx.insert("paginator_series", &paginator.series());
    tmpl_ctx.insert("page", &Page::from(&paginator));
    tmpl_ctx.insert("flat_books_map", &series_flat_books_map);

    ctx
        .template_engine
        .render("container.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

pub async fn get_id(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;

    let series_id = id.into_inner();

    let series = series::Entity::find()
        .filter(series::Column::Id.eq(series_id))
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(
            AnsernoError::NotFound(format!("No record found for Series(id={series_id})"))
                .with_context(&ctx),
        )?;

    let series_books_ids = books_series_link::Entity::find()
        .filter(books_series_link::Column::Series.eq(series.id))
        .select_only()
        .column(books_series_link::Column::Book)
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let slice_paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(series_books_ids.len())
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = slice_paginator
        .selector(
            flat_books_query()
                .filter(books::Column::Id.is_in(series_books_ids))
                .order_by_asc(books::Column::SeriesIndex),
        )
        .into_model::<FlatBook>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("title", &format!("Series - {}", &series.name));
    tmpl_ctx.insert("url", &format!("/series/{}", &series.id));
    tmpl_ctx.insert("flat_books", &flat_books);
    tmpl_ctx.insert("paginator", &slice_paginator);
    tmpl_ctx.insert("paginator_series", &slice_paginator.series());
    tmpl_ctx.insert("page", &Page::from(&slice_paginator));

    ctx
        .template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

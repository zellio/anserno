use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, LoaderTrait, ModelTrait, PaginatorTrait, QueryOrder};

use crate::{
    app::Context,
    entities::{books_series_link, flat_books, series},
    error::{AnsernoError, AnsernoWebResult},
    pagination::{
        substring_buckets, Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder,
        SubstringBucketPaginatorBuilder,
    },
};

pub async fn get(
    ctx: web::Data<Context>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.conn;

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

    let mut flat_books = series
        .load_many_to_many(flat_books::Entity, books_series_link::Entity, conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    for books in flat_books.iter_mut() {
        books.sort_unstable_by(|left, right| {
            left.series_index
                .partial_cmp(&right.series_index)
                .unwrap_or(std::cmp::Ordering::Less)
        });
    }

    let series_flat_books: HashMap<i32, Vec<flat_books::Model>> = series
        .iter()
        .map(|series| series.id)
        .zip(flat_books)
        .collect();

    tmpl_ctx.insert("container", &series);

    tmpl_ctx.insert("paginator", &paginator);
    tmpl_ctx.insert("paginator_series", &paginator.series());
    tmpl_ctx.insert("page", &Page::from(&paginator));
    tmpl_ctx.insert("flat_books_map", &series_flat_books);

    ctx.template_engine
        .render("container.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

pub async fn get_id(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.conn;

    let series_id = id.into_inner();

    let series = series::Entity::find_by_id(series_id)
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(
            AnsernoError::NotFound(format!("No record found for Series(id={series_id})"))
                .with_context(&ctx),
        )?;

    let series_book_count = series
        .find_related(flat_books::Entity)
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let slice_paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(series_book_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = slice_paginator
        .selector(
            series
                .find_related(flat_books::Entity)
                .order_by_asc(flat_books::Column::SeriesIndex),
        )
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

    ctx.template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

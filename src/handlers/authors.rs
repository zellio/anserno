use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{
    app::Context,
    entities::{authors, books, books_authors_link},
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

    tmpl_ctx.insert("title", "Authors");
    tmpl_ctx.insert("url", "/authors");
    tmpl_ctx.insert("include_jump", &true);

    let paginator = SubstringBucketPaginatorBuilder::from(pagination_query.into_inner())
        .bucket_column(authors::Column::Sort)
        .buckets(
            substring_buckets::<authors::Entity, authors::Column, String>(authors::Column::Sort, 1)
                .all(conn)
                .await
                .unwrap_or(vec![]),
        )
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let authors = paginator
        .selector(authors::Entity::find())
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    tmpl_ctx.insert("container", &authors);

    let authors_ids: Vec<i32> = authors.iter().map(|author| author.id).collect();

    let authors_books_ids = books_authors_link::Entity::find()
        .filter(books_authors_link::Column::Author.is_in(authors_ids))
        .select_only()
        .column(books_authors_link::Column::Book)
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let authors_flat_books = get_flat_books_by_id(conn, authors_books_ids)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut authors_flat_books_map = HashMap::new();
    for (author_id, flat_book) in authors_flat_books
        .into_iter()
        .flat_map(|flat_authors_book| {
            flat_authors_book
                .authors
                .as_object()
                .unwrap()
                .into_iter()
                .map(|(author_id, _)| {
                    (
                        author_id.parse::<i32>().unwrap_or(0),
                        flat_authors_book.clone(),
                    )
                })
                .collect::<Vec<(i32, FlatBook)>>()
        })
    {
        authors_flat_books_map
            .entry(author_id)
            .or_insert_with(Vec::new)
            .push(flat_book);
    }

    for books in authors_flat_books_map.values_mut() {
        books.sort_by(|a, b| a.sort.cmp(&b.sort))
    }

    tmpl_ctx.insert("paginator", &paginator);
    tmpl_ctx.insert("paginator_series", &paginator.series());
    tmpl_ctx.insert("page", &Page::from(&paginator));
    tmpl_ctx.insert("flat_books_map", &authors_flat_books_map);

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
    let conn = &ctx.database_connection;

    let author_id = id.into_inner();

    let author = authors::Entity::find()
        .filter(authors::Column::Id.eq(author_id))
        .one(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
        .ok_or(
            AnsernoError::NotFound(format!("No record found for Author(id={author_id})"))
                .with_context(&ctx),
        )?;

    let author_books_ids = books_authors_link::Entity::find()
        .filter(books_authors_link::Column::Author.eq(author.id))
        .select_only()
        .column(books_authors_link::Column::Book)
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let slice_paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(author_books_ids.len())
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = slice_paginator
        .selector(
            flat_books_query()
                .filter(books::Column::Id.is_in(author_books_ids))
                .order_by_asc(books::Column::Sort),
        )
        .into_model::<FlatBook>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("title", &format!("Author - {}", &author.name));
    tmpl_ctx.insert("url", &format!("/authors/{}", &author.id));
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

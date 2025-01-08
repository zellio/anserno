use std::collections::BTreeMap;

use actix_web::{web, HttpResponse, Responder};
use calibre_data::{
    entities::{authors, books_authors_link, flat_books},
    library::CalibreLibrary,
    pagination::{BucketPaginator, QueryPaginator, RecordsQuery},
};
use pagination::paginator::Paginator;
use sea_orm::{EntityTrait, LoaderTrait, ModelTrait, QueryOrder};

use crate::{
    context::Context,
    error::{Error, ResponseResult, WithContext},
    url_params::Pagination,
};

#[actix_web::get("")]
pub async fn get(
    ctx: web::Data<Context>,
    pagination: web::Query<Pagination>,
) -> ResponseResult<impl Responder> {
    let conn = ctx.library().conn();

    let Pagination { page, .. } = pagination.into_inner();

    let bucket_paginator =
        BucketPaginator::from_query(conn, authors::Entity::find(), authors::Column::Sort, 1)
            .await
            .map_err(|err| Error::from(err).with_context(&ctx))?;

    let authors = bucket_paginator
        .records_query(page)
        .all(conn)
        .await
        .map_err(|err| Error::from(err).with_context(&ctx))?;

    let mut flat_books = authors
        .load_many_to_many(flat_books::Entity, books_authors_link::Entity, conn)
        .await
        .map_err(|err| Error::from(err).with_context(&ctx))?;

    for books in flat_books.iter_mut() {
        books.sort_by(|left, right| left.sort.cmp(&right.sort));
    }

    let authors_flat_books: BTreeMap<i32, Vec<flat_books::Model>> = authors
        .iter()
        .map(|author| author.id)
        .zip(flat_books)
        .collect();

    let mut tera_context = tera::Context::new();

    tera_context.insert("title", "Authors");
    tera_context.insert("url", "/authors");
    tera_context.insert("include_jump", &true);

    tera_context.insert("container", &authors);

    tera_context.insert("paginator", &bucket_paginator);
    tera_context.insert("paginator_series", &bucket_paginator.series(page));
    tera_context.insert("paginator_page", &bucket_paginator.page(page));
    tera_context.insert("paginator_items", &0);

    tera_context.insert("flat_books_map", &authors_flat_books);

    ctx.template_engine()
        .render("container.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| Error::from(err).with_context(&ctx))
}

#[actix_web::get("/{id}")]
pub async fn get_id(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
    pagination: web::Query<Pagination>,
) -> ResponseResult<impl Responder> {
    let conn = ctx.library().conn();

    let author_id = id.into_inner();

    let author = authors::Entity::find_by_id(author_id)
        .one(conn)
        .await
        .map_err(|err| err.with_context(&ctx))?
        .ok_or(
            Error::NotFound(format!("No record found for Author(id={author_id})"))
                .with_context(&ctx),
        )?;

    let Pagination { page, items } = pagination.into_inner();

    let query = author
        .find_related(flat_books::Entity)
        .order_by_asc(flat_books::Column::Sort);

    let query_paginator = QueryPaginator::from_query(conn, query)
        .await
        .map_err(|err| err.with_context(&ctx))?
        .with_page_length(items);

    let flat_books = query_paginator
        .records_query(page)
        .all(conn)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut tera_context = tera::Context::new();

    tera_context.insert("title", &format!("Author - {}", author.name));
    tera_context.insert("url", &format!("/authors/{}", author.id));

    tera_context.insert("flat_books", &flat_books);

    tera_context.insert("paginator", &query_paginator);
    tera_context.insert("paginator_series", &query_paginator.series(page));
    tera_context.insert("paginator_page", &query_paginator.page(page));
    tera_context.insert("paginator_items", &items);

    ctx.template_engine()
        .render("list.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| err.with_context(&ctx))
}

pub fn service() -> actix_web::Scope {
    actix_web::Scope::new("/authors")
        .service(get)
        .service(get_id)
}

use crate::{
    app::Context,
    entities::{books, search_index},
    error::{AnsernoError, AnsernoWebResult},
    pagination::{Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder},
    queries::{flat_books_query, search_query, FlatBook},
};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQueryParams {
    query: String,
}

pub async fn get(
    ctx: web::Data<Context>,
    query: web::Query<SearchQueryParams>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;
    let query = query.into_inner().query;

    let search_count = search_query(&query)
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(search_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let book_ids = paginator
        .selector(
            search_query(&query)
                .select_only()
                .column(search_index::Column::BookId)
                .order_by_asc(search_index::Column::Sort),
        )
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let flat_books = flat_books_query()
        .filter(books::Column::Id.is_in(book_ids))
        .into_model::<FlatBook>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("title", "Search Results");
    tmpl_ctx.insert("url", &format!("/search?query={}", &query));

    tmpl_ctx.insert("flat_books", &flat_books);
    tmpl_ctx.insert("paginator", &paginator);
    tmpl_ctx.insert("paginator_series", &paginator.series());
    tmpl_ctx.insert("page", &Page::from(&paginator));

    ctx
        .template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

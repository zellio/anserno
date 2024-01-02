use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    sea_query::Expr, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::Context,
    entities::{flat_books, search_index},
    error::{AnsernoError, AnsernoWebResult},
    pagination::{Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder},
    queries::SelectAlias,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQueryParams {
    query: String,
}

pub async fn get(
    ctx: web::Data<Context>,
    query: web::Query<SearchQueryParams>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.conn;
    let query = query.into_inner().query;

    let search_query = search_index::Entity::find()
        .filter(Expr::col(SelectAlias("anserno_search_index")).eq(&query))
        .select_only()
        .column(search_index::Column::BookId)
        .order_by_asc(search_index::Column::Sort);

    let search_count = search_query
        .clone()
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(search_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let book_ids = paginator
        .selector(search_query)
        .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let flat_books = flat_books::Entity::find()
        .filter(flat_books::Column::Id.is_in(book_ids))
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

    ctx.template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

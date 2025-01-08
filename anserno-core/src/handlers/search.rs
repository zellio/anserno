use actix_web::{web, HttpResponse, Responder};
use calibre_data::{
    entities::{flat_books, search_index},
    library::CalibreLibrary,
    pagination::{QueryPaginator, RecordsQuery},
    query::select_alias::SelectAlias,
};
use pagination::paginator::Paginator;
use sea_orm::{prelude::Expr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{
    context::Context,
    error::{ResponseResult, WithContext},
    url_params,
};

#[actix_web::get("")]
pub async fn get(
    ctx: web::Data<Context>,
    query: web::Query<url_params::Search>,
    pagination: web::Query<url_params::Pagination>,
) -> ResponseResult<impl Responder> {
    let conn = ctx.library().conn();

    let query = &query.into_inner().query;

    let url_params::Pagination { page, items } = pagination.into_inner();

    let search_query = search_index::Entity::find()
        .filter(Expr::col(SelectAlias("anserno_search_index")).eq(query))
        .select_only()
        .column(search_index::Column::BookId)
        .order_by_asc(search_index::Column::Sort);

    let paginator = QueryPaginator::from_query(conn, search_query)
        .await
        .map_err(|err| err.with_context(&ctx))?
        .with_page_length(items);

    let search_results = paginator
        .records_query(page)
        .find_with_related(flat_books::Entity)
        // .into_tuple::<i32>()
        .all(conn)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = search_results
        .into_iter()
        .flat_map(|(_, books)| books)
        .collect::<Vec<_>>();

    let mut tera_context = tera::Context::new();

    tera_context.insert("title", "Search Results");
    tera_context.insert("url", &format!("/search?query={}", query));

    tera_context.insert("flat_books", &flat_books);

    tera_context.insert("paginator", &paginator);
    tera_context.insert("paginator_series", &paginator.series(page));
    tera_context.insert("paginator_page", &paginator.page(page));
    tera_context.insert("paginator_items", &items);

    ctx.template_engine()
        .render("list.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| err.with_context(&ctx))
}

pub fn service() -> actix_web::Scope {
    actix_web::Scope::new("/search").service(get)
}

use actix_web::{web, HttpResponse, Responder};
use calibre_data::{entities::flat_books, library::CalibreLibrary};
use sea_orm::{
    sea_query::{Func, SimpleExpr},
    EntityTrait, QueryOrder, QuerySelect,
};

use crate::{
    context::Context,
    error::{Error, ResponseError, ResponseResult, WithContext},
    url_params::Pagination,
};

#[actix_web::get("")]
pub async fn get(
    ctx: web::Data<Context>,
    pagination: web::Query<Pagination>,
) -> ResponseResult<impl Responder> {
    let Pagination { items, .. } = pagination.into_inner();

    let flat_books = flat_books::Entity::find()
        .order_by_asc(SimpleExpr::FunctionCall(Func::random()))
        .limit(items)
        .all(ctx.library().conn())
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut tera_context = tera::Context::new();

    tera_context.insert("flat_books", &flat_books);

    ctx.template_engine()
        .render("list.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| err.with_context(&ctx))
}

pub fn service() -> actix_web::Scope {
    actix_web::Scope::new("/").service(get)
}

pub async fn default_service(ctx: web::Data<Context>) -> ResponseResult<impl Responder> {
    Err::<HttpResponse, ResponseError>(Error::NotFound("".to_string()).with_context(&ctx))
}

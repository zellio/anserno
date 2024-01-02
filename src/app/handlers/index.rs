use actix_web::{web, HttpResponse, Responder};
use sea_orm::{
    sea_query::{Func, SimpleExpr},
    EntityTrait, QueryOrder, QuerySelect,
};

use crate::{
    app::Context,
    entities::flat_books,
    error::{AnsernoError, AnsernoWebError, AnsernoWebResult},
};

pub async fn get(ctx: web::Data<Context>) -> AnsernoWebResult<impl Responder> {
    let flat_books = flat_books::Entity::find()
        .order_by_asc(SimpleExpr::FunctionCall(Func::random()))
        .limit(12u64)
        .all(&ctx.conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();
    tmpl_ctx.insert("flat_books", &flat_books);

    ctx.template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

pub async fn default_service(ctx: web::Data<Context>) -> AnsernoWebResult<impl Responder> {
    Err::<HttpResponse, AnsernoWebError>(AnsernoError::NotFound("".to_string()).with_context(&ctx))
}

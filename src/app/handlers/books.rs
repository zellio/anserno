use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use url::Origin;

use crate::{
    app::Context,
    entities::flat_books,
    error::{AnsernoError, AnsernoWebResult},
    pagination::{Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder},
};

pub async fn get(
    ctx: web::Data<Context>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.conn;

    let books_count = flat_books::Entity::find()
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(books_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = paginator
        .selector(flat_books::Entity::find().order_by_desc(flat_books::Column::Id))
        .all(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("title", "Books");
    tmpl_ctx.insert("url", "/books");

    tmpl_ctx.insert("flat_books", &flat_books);
    tmpl_ctx.insert("paginator", &paginator);
    tmpl_ctx.insert("paginator_series", &paginator.series());
    tmpl_ctx.insert("page", &Page::from(&paginator));

    ctx.template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

async fn get_flat_book_by_id(
    ctx: &web::Data<Context>,
    id: i32,
) -> AnsernoWebResult<flat_books::Model> {
    flat_books::Entity::find_by_id(id)
        .one(&ctx.conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(ctx))?
        .ok_or(AnsernoError::NotFound("".to_string()).with_context(ctx))
}

pub async fn get_id(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let flat_book = get_flat_book_by_id(&ctx, id.into_inner()).await?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("flat_book", &flat_book);

    ctx.template_engine
        .render("books/id.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

async fn get_flat_book_file(
    ctx: &web::Data<Context>,
    req: &HttpRequest,
    id: i32,
    filename: &str,
) -> AnsernoWebResult<impl Responder> {
    let flat_book = get_flat_book_by_id(ctx, id).await?;

    let file_url = ctx
        .library
        .flat_book_file_url(flat_book, filename)
        .map_err(|err| err.with_context(ctx))?;

    match file_url.origin() {
        Origin::Opaque(_) => {
            let file_path = file_url
                .to_file_path()
                .map_err(|err| AnsernoError::from(err).with_context(ctx))?;

            NamedFile::open(file_path)
                .map(|named_file| {
                    named_file
                        .use_etag(true)
                        .use_last_modified(true)
                        .into_response(req)
                })
                .map_err(|err| AnsernoError::from(err).with_context(ctx))
        }

        Origin::Tuple(_, _, _) => Ok(HttpResponse::SeeOther()
            .insert_header(("location", file_url.as_str()))
            .finish()),
    }
}

pub async fn get_id_cover(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    get_flat_book_file(&ctx, &req, id.into_inner(), "cover.jpg").await
}

pub async fn get_id_thumb(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    get_flat_book_file(&ctx, &req, id.into_inner(), "thumb.jpg").await
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadPathParams {
    id: i32,
    format: String,
}

pub async fn get_id_download_format(
    ctx: web::Data<Context>,
    req: HttpRequest,
    params: web::Path<DownloadPathParams>,
) -> AnsernoWebResult<impl Responder> {
    let DownloadPathParams { id, format } = params.into_inner();
    let flat_book = get_flat_book_by_id(&ctx, id).await?;

    let formats = flat_book
        .formats
        .as_object()
        .map(serde_json::Map::to_owned)
        .ok_or(
            AnsernoError::NotFound(format!("No formats found for Book(id={id})"))
                .with_context(&ctx),
        )?;

    let filename = formats
        .get(&format.to_uppercase())
        .and_then(serde_json::Value::as_str)
        .map(|format_filename| format!("{format_filename}.{}", format.to_lowercase()))
        .ok_or(
            AnsernoError::NotFound(format!(
                "Specified format {format} not found for Book(id={id})"
            ))
            .with_context(&ctx),
        )?;

    get_flat_book_file(&ctx, &req, id, &filename).await
}

pub async fn get_id_read(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let flat_book = get_flat_book_by_id(&ctx, id.into_inner()).await?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("flat_book", &flat_book);

    ctx.template_engine
        .render("books/id/read.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

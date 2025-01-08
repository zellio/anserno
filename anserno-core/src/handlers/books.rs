use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calibre_data::{
    entities::flat_books,
    library::CalibreLibrary,
    pagination::{QueryPaginator, RecordsQuery},
};
use pagination::paginator::Paginator;
use sea_orm::{EntityTrait, QueryOrder};
use url::Origin;

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

    let Pagination { page, items } = pagination.into_inner();

    let query = flat_books::Entity::find().order_by_desc(flat_books::Column::Id);

    let paginator = QueryPaginator::from_query(conn, query)
        .await
        .map_err(|err| err.with_context(&ctx))?
        .with_page_length(items);

    let flat_books = paginator
        .records_query(page)
        .all(conn)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut tera_context = tera::Context::new();

    tera_context.insert("title", "Books");
    tera_context.insert("url", "/books");

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

async fn find_flat_book(
    ctx: &web::Data<Context>,
    flat_book_id: i32,
) -> ResponseResult<flat_books::Model> {
    flat_books::Entity::find_by_id(flat_book_id)
        .one(ctx.library().conn())
        .await
        .map_err(|err| err.with_context(ctx))?
        .ok_or(
            Error::NotFound(format!("No record found for Book(id={flat_book_id})"))
                .with_context(ctx),
        )
}

#[actix_web::get("/{id}")]
pub async fn get_id(ctx: web::Data<Context>, id: web::Path<i32>) -> ResponseResult<impl Responder> {
    let flat_book = find_flat_book(&ctx, id.into_inner()).await?;

    let mut tera_context = tera::Context::new();

    tera_context.insert("flat_book", &flat_book);

    ctx.template_engine()
        .render("books/id.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| err.with_context(&ctx))
}

async fn flat_book_file(
    ctx: &web::Data<Context>,
    req: &HttpRequest,
    flat_book_id: i32,
    filename: &str,
) -> ResponseResult<impl Responder> {
    let flat_book = find_flat_book(ctx, flat_book_id).await?;

    let file_resource = ctx
        .library()
        .flat_book_resource_path(&flat_book, filename)
        .map_err(|err| err.with_context(ctx))?;

    match file_resource.origin() {
        Origin::Opaque(_) => {
            let path = file_resource
                .to_file_path()
                .map_err(|_| Error::Unknown.with_context(ctx))?;

            NamedFile::open(path)
                .map(|named_file| {
                    named_file
                        .use_etag(true)
                        .use_last_modified(true)
                        .into_response(req)
                })
                .map_err(|err| err.with_context(ctx))
        }

        Origin::Tuple(_, _, _) => Ok(HttpResponse::SeeOther()
            .insert_header(("location", file_resource.as_str()))
            .finish()),
    }
}

#[actix_web::get("/{id}/cover")]
pub async fn get_id_cover(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> ResponseResult<impl Responder> {
    flat_book_file(&ctx, &req, id.into_inner(), "cover.jpg").await
}

#[actix_web::get("/{id}/thumb")]
pub async fn get_id_thumb(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> ResponseResult<impl Responder> {
    flat_book_file(&ctx, &req, id.into_inner(), "thumb.jpg").await
}

#[derive(serde::Deserialize)]
pub struct DownloadRequest {
    id: i32,
    format: String,
}

#[actix_web::get("/{id}/download/{format}")]
pub async fn get_id_download_format(
    ctx: web::Data<Context>,
    req: HttpRequest,
    download_request: web::Path<DownloadRequest>,
) -> ResponseResult<impl Responder> {
    let DownloadRequest { id, format } = download_request.into_inner();

    let flat_book = find_flat_book(&ctx, id).await?;

    let formats = flat_book
        .formats
        .as_object()
        .map(serde_json::Map::to_owned)
        .ok_or(
            Error::NotFound(format!(
                "Cannot locate flat_book formts for book {}",
                flat_book.id,
            ))
            .with_context(&ctx),
        )?;

    let format_path = formats
        .get(&format.to_uppercase())
        .and_then(serde_json::Value::as_str)
        .map(|filename| format!("{filename}.{}", format.to_lowercase()))
        .ok_or(
            Error::NotFound(format!(
                "Cannot locate flat_book formt {} for book {}",
                format, flat_book.id,
            ))
            .with_context(&ctx),
        )?;

    flat_book_file(&ctx, &req, flat_book.id, &format_path).await
}

#[actix_web::get("/{id}/read")]
pub async fn get_id_read(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> ResponseResult<impl Responder> {
    let flat_book = find_flat_book(&ctx, id.into_inner()).await?;
    let mut tera_context = tera::Context::new();

    tera_context.insert("flat_book", &flat_book);

    ctx.template_engine()
        .render("books/id/read.html", &tera_context)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| err.with_context(&ctx))
}

pub fn service() -> actix_web::Scope {
    actix_web::Scope::new("/books")
        .service(get)
        .service(get_id)
        .service(get_id_cover)
        .service(get_id_thumb)
        .service(get_id_download_format)
        .service(get_id_read)
}

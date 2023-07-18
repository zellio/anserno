use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    app::Context,
    entities::books,
    error::{AnsernoError, AnsernoWebResult},
    pagination::{Page, PaginationUrlQueryParams, Paginator, SlicePaginatorBuilder},
    queries::{flat_books_query, get_flat_book_by_id, FlatBook},
};

pub async fn get(
    ctx: web::Data<Context>,
    pagination_query: web::Query<PaginationUrlQueryParams>,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;

    let books_count = books::Entity::find()
        .count(conn)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let paginator = SlicePaginatorBuilder::from(pagination_query.into_inner())
        .count(books_count as usize)
        .build()
        .map_err(|err| err.with_context(&ctx))?;

    let flat_books = paginator
        .selector(flat_books_query().order_by_desc(books::Column::Id))
        .into_model::<FlatBook>()
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

    ctx
        .template_engine
        .render("list.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

pub async fn get_id(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let flat_book = flat_books_query()
        .order_by_desc(books::Column::Id)
        .filter(books::Column::Id.eq(id.into_inner()))
        .into_model::<FlatBook>()
        .one(&ctx.database_connection)
        .await
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();

    tmpl_ctx.insert("flat_book", &flat_book);

    ctx
        .template_engine
        .render("books/id.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

fn get_flat_book_file(
    ctx: web::Data<Context>,
    req: HttpRequest,
    flat_book: &FlatBook,
    filename: &str,
) -> AnsernoWebResult<impl Responder> {
    let library = &ctx.library;
    let path = library.flat_book_path(flat_book, filename);
    if library.is_local() {
        Ok(NamedFile::open(path)
            .map(|named_file| {
                named_file
                    .use_etag(true)
                    .use_last_modified(true)
                    .into_response(&req)
            })
            .map_err(|err| AnsernoError::from(err).with_context(&ctx))?)
    } else {
        Ok(HttpResponse::SeeOther()
            .insert_header(("location", path))
            .finish())
    }
}

async fn get_id_file(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
    filename: &str,
) -> AnsernoWebResult<impl Responder> {
    let conn = &ctx.database_connection;
    let id = id.into_inner();

    let flat_book = &get_flat_book_by_id(conn, id)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    get_flat_book_file(ctx, req, flat_book, filename)
}

pub async fn get_id_cover(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    get_id_file(ctx, req, id, "cover.jpg").await
}

pub async fn get_id_thumb(
    ctx: web::Data<Context>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    get_id_file(ctx, req, id, "thumb.jpg").await
}

pub async fn get_id_download_format(
    ctx: web::Data<Context>,
    req: HttpRequest,
    params: web::Path<(i32, String)>,
) -> AnsernoWebResult<impl Responder> {
    let (id, format) = params.into_inner();

    tracing::error!("{:?} {:?}", &id, &format);

    let flat_book = get_flat_book_by_id(&ctx.database_connection, id)
        .await
        .map_err(|err| err.with_context(&ctx))?;

    tracing::error!("{:?}", &flat_book);

    let format_fname = ctx
        .library
        .flat_book_format_fname(&flat_book, format.as_str())
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))?;

    tracing::error!("{:?}", &format_fname);

    get_flat_book_file(ctx, req, &flat_book, &format_fname)
}

pub async fn get_id_read(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> AnsernoWebResult<impl Responder> {
    let flat_book = get_flat_book_by_id(&ctx.database_connection, id.into_inner())
        .await
        .map_err(|err| err.with_context(&ctx))?;

    let mut tmpl_ctx = tera::Context::new();
    tmpl_ctx.insert("flat_book", &flat_book);

    ctx
        .template_engine
        .render("books/id/read.html", &tmpl_ctx)
        .map(|body| HttpResponse::Ok().body(body))
        .map_err(|err| AnsernoError::from(err).with_context(&ctx))
}

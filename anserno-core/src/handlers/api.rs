use actix_web::{web, HttpResponse, Responder};
use calibre_data::{
    entities::{authors, books, series},
    library::CalibreLibrary,
    pagination::{QueryPaginator, RecordsQuery},
};
use hypertext_application_language::{ext::sea_orm::AsResource, link::Link, resource::Resource};
use pagination::{config::Config, paginator::Paginator};

use sea_orm::{EntityTrait, PrimaryKeyTrait};

use crate::{
    context::Context,
    error::{Error, JsonResponseResult, ToJsonError},
    url_params::Pagination,
};

#[actix_web::get("")]
pub async fn get_root() -> impl Responder {
    web::Json(
        Resource::default()
            .with_links(
                "authors",
                [
                    Link::new("/authors").with_title("authors"),
                    Link::new("/authors/{id}")
                        .with_title("author")
                        .with_templated(true),
                ],
            )
            .with_links(
                "books",
                [
                    Link::new("/books").with_title("books"),
                    Link::new("/books/{id}")
                        .with_title("book")
                        .with_templated(true),
                ],
            )
            .with_links(
                "series",
                [
                    Link::new("/series").with_title("series"),
                    Link::new("/series/{id}")
                        .with_title("series")
                        .with_templated(true),
                ],
            ),
    )
}

pub async fn get<E>(
    ctx: web::Data<Context>,
    pagination: web::Query<Pagination>,
) -> JsonResponseResult<impl Responder>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: ::core::marker::Sync + AsResource,
{
    let conn = ctx.library().conn();
    let Pagination { items, page } = pagination.into_inner();

    let paginator = QueryPaginator::from_query(conn, E::find())
        .await
        .map_err(ToJsonError::to_json_error)?
        .with_page_length(items);

    let records = paginator
        .records_query(page)
        .all(conn)
        .await
        .map_err(ToJsonError::to_json_error)?;

    let record = records
        .first()
        .ok_or(Error::NotFound("No records found".to_string()).to_json_error())?;

    let mut resource = Resource::default().with_link("self", record.list_link(page, items));

    let paginator_page = paginator.page(page);

    if let Some(prev) = paginator_page.previous() {
        resource = resource.with_link("prev", record.list_link(prev, items));
    }

    if let Some(next) = paginator_page.next() {
        resource = resource.with_link("prev", record.list_link(next, items));
    }

    Ok(web::Json(
        resource
            .with_property("page", page)
            .with_property("pages", Config::last(paginator.config()))
            .with_property("count", records.len())
            .with_embeddeds(
                "items",
                records
                    .iter()
                    .map(Resource::from_model::<E>)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(ToJsonError::to_json_error)?,
            ),
    ))
}

pub async fn get_id<E>(
    ctx: web::Data<Context>,
    id: web::Path<i32>,
) -> JsonResponseResult<impl Responder>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: ::core::marker::Sync + AsResource,
    <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
{
    let conn = ctx.library().conn();

    let id = id.into_inner();

    let model = E::find_by_id(id)
        .one(conn)
        .await
        .map_err(ToJsonError::to_json_error)?
        .ok_or(Error::NotFound(format!(
            "No record found for Record(id={id})"
        )))
        .map_err(ToJsonError::to_json_error)?;

    model
        .as_resource(conn)
        .await
        .map(web::Json)
        .map_err(ToJsonError::to_json_error)
}

pub fn entity_service<E>(name: &str) -> actix_web::Scope
where
    E: EntityTrait,
    <E as EntityTrait>::Model: ::core::marker::Sync + AsResource,
    <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
{
    web::scope(&format!("/{name}"))
        .service(web::resource([""]).route(web::get().to(get::<E>)))
        .service(web::resource(["/{id}"]).route(web::get().to(get_id::<E>)))
}

pub async fn api_redirect() -> impl Responder {
    HttpResponse::SeeOther()
        .insert_header(("location", "/api"))
        .finish()
}

pub fn service() -> actix_web::Scope {
    web::scope("/api")
        .service(get_root)
        .service(entity_service::<authors::Entity>("authors"))
        .service(entity_service::<books::Entity>("books"))
        .service(entity_service::<series::Entity>("series"))
        .service(web::scope("/0.1.0").default_service(web::to(api_redirect)))
}

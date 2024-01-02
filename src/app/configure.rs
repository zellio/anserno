use actix_web::web;

use crate::{
    app::handlers,
    entities::{authors, books, series},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/0.1.0")
            .service(web::resource([""]).route(web::get().to(handlers::api::get_index)))
            .service(
                web::scope("/authors")
                    .service(
                        web::resource([""])
                            .route(web::get().to(handlers::api::get::<authors::Entity>)),
                    )
                    .service(
                        web::resource(["/{id}"])
                            .route(web::get().to(handlers::api::get_by_id::<authors::Entity>)),
                    ),
            )
            .service(
                web::scope("/books")
                    .service(
                        web::resource([""])
                            .route(web::get().to(handlers::api::get::<books::Entity>)),
                    )
                    .service(
                        web::resource(["/{id}"])
                            .route(web::get().to(handlers::api::get_by_id::<books::Entity>)),
                    ),
            )
            .service(
                web::scope("/series")
                    .service(
                        web::resource([""])
                            .route(web::get().to(handlers::api::get::<series::Entity>)),
                    )
                    .service(
                        web::resource(["/{id}"])
                            .route(web::get().to(handlers::api::get_by_id::<series::Entity>)),
                    ),
            ),
    )
    .service(
        web::scope("/").service(web::resource([""]).route(web::get().to(handlers::index::get))),
    )
    .service(
        web::scope("/authors")
            .service(web::resource([""]).route(web::get().to(handlers::authors::get)))
            .service(web::resource(["/{id}"]).route(web::get().to(handlers::authors::get_id))),
    )
    .service(
        web::scope("/books")
            .service(web::resource([""]).route(web::get().to(handlers::books::get)))
            .service(web::resource(["/{id}"]).route(web::get().to(handlers::books::get_id)))
            .service(
                web::resource(["/{id}/cover"]).route(web::get().to(handlers::books::get_id_cover)),
            )
            .service(
                web::resource(["/{id}/thumb"]).route(web::get().to(handlers::books::get_id_thumb)),
            )
            .service(
                web::resource(["/{id}/download/{format}", "/{id}/download.{format}"])
                    .route(web::get().to(handlers::books::get_id_download_format)),
            )
            .service(
                web::resource(["/{id}/read"]).route(web::get().to(handlers::books::get_id_read)),
            ),
    )
    .service(
        web::scope("/search")
            .service(web::resource([""]).route(web::get().to(handlers::search::get))),
    )
    .service(
        web::scope("/series")
            .service(web::resource([""]).route(web::get().to(handlers::series::get)))
            .service(web::resource(["/{id}"]).route(web::get().to(handlers::series::get_id))),
    )
    .service(web::scope("/static").service(
        web::resource(["/{kind}/{file}"]).route(web::get().to(handlers::static_files::get)),
    ))
    .default_service(web::to(handlers::index::default_service));

    // #[cfg(feature = "search")]
    // cfg
}

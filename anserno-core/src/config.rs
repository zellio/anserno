use actix_web::web;

use crate::handlers::{api, authors, books, index, search, series, static_files};

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(api::service())
        .service(authors::service())
        .service(books::service())
        .service(search::service())
        .service(series::service())
        .service(static_files::service())
        .service(index::get)
        .service(index::get_robots_txt)
        .default_service(web::to(index::default_service));
}

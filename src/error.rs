use std::io::ErrorKind;

// use crate::app::Context;
use actix_web::web;
// use derive_builder;
// use std::io::ErrorKind;

use crate::app::Context;

#[derive(Debug)]
pub enum AnsernoError {
    Builder(derive_builder::UninitializedFieldError),
    NoDatabase,
    NotFound(String),
    NotAllowed(String),
    Reqwest(reqwest::Error),
    SeaDb(sea_orm::DbErr),
    StdIo(std::io::Error),
    Tera(tera::Error),
    Unknown(String),
    //     NotFound(String),
    //     NotAllowed(String),
    //     JhalBuilder(String),
}

impl std::fmt::Display for AnsernoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builder(err) => write!(f, "Builder error: {err}"),
            Self::NoDatabase => write!(f, "No database found"),
            Self::NotFound(msg) => write!(f, "Not Found error: {msg}"),
            Self::NotAllowed(msg) => write!(f, "Not Allowed error: {msg}"),
            Self::Reqwest(err) => write!(f, "Reqwest error: {err}"),
            Self::SeaDb(err) => write!(f, "Sea Db error: {err}"),
            Self::StdIo(err) => write!(f, "IO error: {err}"),
            Self::Tera(err) => write!(f, "Tera error: {err}"),
            Self::Unknown(msg) => write!(f, "Unknown error: {msg}"),
            //             Self::JhalBuilder(msg) => write!(f, "JhalBuilder error: {msg}"),
            //             Self::NotFound(msg) => write!(f, "Not Found error: {msg}"),
            //             Self::NotAllowed(msg) => write!(f, "Not Allowed error: {msg}"),
        }
    }
}

impl std::error::Error for AnsernoError {}

impl AnsernoError {
    pub fn with_context(self, ctx: &web::Data<Context>) -> AnsernoWebError {
        AnsernoWebError {
            ctx: ctx.clone(),
            err: self,
        }
    }
}

#[derive(Debug)]
pub struct AnsernoWebError {
    ctx: web::Data<Context>,
    err: AnsernoError,
}

impl std::fmt::Display for AnsernoWebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.err, f)
    }
}

impl std::error::Error for AnsernoWebError {}

impl actix_web::ResponseError for AnsernoWebError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self.err {
            AnsernoError::StdIo(err) => match err.kind() {
                ErrorKind::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                ErrorKind::ConnectionRefused | ErrorKind::PermissionDenied => {
                    actix_web::http::StatusCode::METHOD_NOT_ALLOWED
                }
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            },

            AnsernoError::Reqwest(err) => err
                .status()
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),

            AnsernoError::NotAllowed(_) => actix_web::http::StatusCode::METHOD_NOT_ALLOWED,

            AnsernoError::NotFound(_) | AnsernoError::SeaDb(sea_orm::DbErr::RecordNotFound(_)) => {
                actix_web::http::StatusCode::NOT_FOUND
            }

            AnsernoError::Unknown(_)
            // | AnsernoError::JhalBuilder(_)
            | AnsernoError::NoDatabase
            | AnsernoError::Builder(_)
            | AnsernoError::Tera(_)
            | AnsernoError::SeaDb(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();

        let mut tmpl_ctx = tera::Context::new();

        tmpl_ctx.insert("title", "Error");
        tmpl_ctx.insert("status", &status_code.as_u16());
        tmpl_ctx.insert("error", &format!("{}", self.err));

        let mut builder = actix_web::HttpResponse::build(self.status_code());

        builder
            .insert_header(actix_web::http::header::ContentType::html())
            .body(
                self.ctx
                    .template_engine
                    .render("error.html", &tmpl_ctx)
                    .unwrap(),
            )
    }
}

impl From<()> for AnsernoError {
    fn from(_: ()) -> Self {
        AnsernoError::Unknown("Unit Error".to_string())
    }
}

impl From<derive_builder::UninitializedFieldError> for AnsernoError {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        AnsernoError::Builder(value)
    }
}

impl From<reqwest::Error> for AnsernoError {
    fn from(value: reqwest::Error) -> Self {
        AnsernoError::Reqwest(value)
    }
}

impl From<sea_orm::DbErr> for AnsernoError {
    fn from(value: sea_orm::DbErr) -> Self {
        AnsernoError::SeaDb(value)
    }
}

impl From<std::io::Error> for AnsernoError {
    fn from(value: std::io::Error) -> Self {
        AnsernoError::StdIo(value)
    }
}

impl From<tera::Error> for AnsernoError {
    fn from(value: tera::Error) -> Self {
        AnsernoError::Tera(value)
    }
}

pub type AnsernoResult<T> = core::result::Result<T, AnsernoError>;

pub type AnsernoWebResult<T> = core::result::Result<T, AnsernoWebError>;

mod response_error;
pub use response_error::*;

mod json_response_error;
pub use json_response_error::*;

#[derive(Debug)]
pub enum Error {
    CalibreData(calibre_data::error::Error),
    DbErr(sea_orm::DbErr),
    Forbidden(String),
    HypertextApplicationLanguage(hypertext_application_language::error::Error),
    NotFound(String),
    StdIo(::std::io::Error),
    Tera(tera::Error),
    Unknown,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "anserno-core: ")?;

        match self {
            Self::CalibreData(err) => err.fmt(f),
            Self::DbErr(err) => err.fmt(f),
            Self::Forbidden(msg) => write!(f, "Forbidden: {msg}"),
            Self::HypertextApplicationLanguage(err) => err.fmt(f),
            Self::NotFound(msg) => write!(f, "NotFound: {msg}"),
            Self::StdIo(err) => err.fmt(f),
            Self::Tera(err) => err.fmt(f),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl crate::error::StatusCode for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
            Self::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            Self::CalibreData(_)
            | Self::HypertextApplicationLanguage(_)
            | Self::StdIo(_)
            | Self::DbErr(_)
            | Self::Tera(_)
            | Self::Unknown => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<calibre_data::error::Error> for Error {
    fn from(value: calibre_data::error::Error) -> Self {
        Self::CalibreData(value)
    }
}

impl From<hypertext_application_language::error::Error> for Error {
    fn from(value: hypertext_application_language::error::Error) -> Self {
        Self::HypertextApplicationLanguage(value)
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Self {
        Self::StdIo(value)
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::DbErr(value)
    }
}

impl From<tera::Error> for Error {
    fn from(value: tera::Error) -> Self {
        Self::Tera(value)
    }
}

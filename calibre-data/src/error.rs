#[derive(Debug)]
pub enum Error {
    DbErr(sea_orm::DbErr),
    RemoteLibrary(String),
    Reqwest(reqwest::Error),
    StdIo(::std::io::Error),
    Unknown,
    UrlParse(url::ParseError),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "calibre-data: ")?;
        match self {
            Self::DbErr(err) => err.fmt(f),
            Self::RemoteLibrary(msg) => write!(f, "RemoteLibrary Error: {msg}"),
            Self::Reqwest(err) => err.fmt(f),
            Self::StdIo(err) => err.fmt(f),
            Self::Unknown => write!(f, "Unknown error"),
            Self::UrlParse(err) => err.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::DbErr(value)
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Self {
        Self::StdIo(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParse(value)
    }
}

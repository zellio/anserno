#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    Unknown,

    #[cfg(feature = "sea-orm")]
    DbErr(sea_orm::DbErr),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hypertext-application-langauge: ")?;

        match self {
            Self::SerdeJson(err) => err.fmt(f),
            Self::Unknown => write!(f, "Unknown error"),

            #[cfg(feature = "sea-orm")]
            Self::DbErr(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

#[cfg(feature = "sea-orm")]
impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::DbErr(value)
    }
}

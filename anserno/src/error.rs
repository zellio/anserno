#[derive(Debug)]
pub enum Error {
    CalibreData(calibre_data::error::Error),
    StdIo(::std::io::Error),
    UrlParse(url::ParseError),
    Unknown,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "anserno-core: ")?;

        match self {
            Self::CalibreData(err) => err.fmt(f),
            Self::StdIo(err) => err.fmt(f),
            Self::UrlParse(err) => err.fmt(f),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl ::std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<calibre_data::error::Error> for Error {
    fn from(value: calibre_data::error::Error) -> Self {
        Self::CalibreData(value)
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Self {
        Self::StdIo(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParse(value)
    }
}

#[derive(::std::fmt::Debug, serde::Serialize)]
pub struct JsonResponseError {
    title: String,
    status: u16,
    error: String,
}

impl ::std::fmt::Display for JsonResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl ::std::error::Error for JsonResponseError {}

// TODO(https://github.com/rust-lang/rust/issues/112792): Remove this allow
#[allow(type_alias_bounds)]
pub type JsonResponseResult<R: actix_web::Responder> = ::std::result::Result<R, JsonResponseError>;

impl<E> From<E> for JsonResponseError
where
    E: crate::error::StatusCode + ::std::fmt::Display,
{
    fn from(value: E) -> Self {
        Self {
            title: "error".to_string(),
            status: value.status_code().as_u16(),
            error: value.to_string(),
        }
    }
}

impl actix_web::ResponseError for JsonResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status).unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponseBuilder::new(self.status_code())
            .insert_header(actix_web::http::header::ContentType::json())
            .body(serde_json::to_string(self).unwrap())
    }
}

pub trait ToJsonError {
    fn to_json_error(self) -> JsonResponseError;
}

impl<E> ToJsonError for E
where
    crate::error::Error: From<E>,
{
    fn to_json_error(self) -> JsonResponseError {
        crate::error::Error::from(self).into()
    }
}

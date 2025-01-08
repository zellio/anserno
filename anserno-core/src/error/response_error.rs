#[derive(Debug)]
pub struct ResponseError {
    error: crate::error::Error,
    context: actix_web::web::Data<crate::context::Context>,
}

impl ::std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl ::std::error::Error for ResponseError {}

// TODO(https://github.com/rust-lang/rust/issues/112792): Remove this allow
#[allow(type_alias_bounds)]
pub type ResponseResult<R: actix_web::Responder> = ::std::result::Result<R, ResponseError>;

pub trait StatusCode {
    fn status_code(&self) -> actix_web::http::StatusCode;
}

impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();

        let mut tera_context = tera::Context::new();

        tera_context.insert("title", "Error");
        tera_context.insert("status", &status_code.as_u16());
        tera_context.insert("error", &self.error.to_string());

        actix_web::HttpResponse::build(status_code)
            .insert_header(actix_web::http::header::ContentType::html())
            .body(
                self.context
                    .template_engine()
                    .render("error.html", &tera_context)
                    .unwrap(),
            )
    }
}

pub trait WithContext {
    fn with_context(self, ctx: &actix_web::web::Data<crate::context::Context>) -> ResponseError;
}

impl<E> WithContext for E
where
    crate::error::Error: From<E>,
{
    fn with_context(self, ctx: &actix_web::web::Data<crate::context::Context>) -> ResponseError {
        ResponseError {
            error: crate::error::Error::from(self),
            context: ctx.clone(),
        }
    }
}

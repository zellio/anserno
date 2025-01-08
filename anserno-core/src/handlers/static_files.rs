use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Responder};

use crate::{
    context::Context,
    error::{Error, ResponseResult, WithContext},
};

#[actix_web::get("/{kind}/{file}")]
pub async fn get(
    ctx: web::Data<Context>,
    req: HttpRequest,
    params: web::Path<(String, String)>,
) -> ResponseResult<impl Responder> {
    let (kind, name) = params.into_inner();

    match kind.as_str() {
        "style" | "script" => {
            let path = ctx.static_files_dir().join(kind).join(name);
            if path.is_file() {
                Ok(NamedFile::open(path)
                    .map_err(|err| err.with_context(&ctx))?
                    .into_response(&req))
            } else {
                Err(
                    Error::NotFound(format!("No file: {}", path.to_str().unwrap_or_default()))
                        .with_context(&ctx),
                )
            }
        }

        _ => Err(Error::Forbidden(format!("Invalid static kind: {kind}")).with_context(&ctx)),
    }
}

pub fn service() -> actix_web::Scope {
    actix_web::Scope::new("/static").service(get)
}

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Responder};

use crate::{
    app::Context,
    error::{AnsernoError, AnsernoWebResult},
};

pub async fn get(
    ctx: web::Data<Context>,
    req: HttpRequest,
    params: web::Path<(String, String)>,
) -> AnsernoWebResult<impl Responder> {
    let (kind, name) = params.into_inner();

    match kind.as_str() {
        "style" | "script" => {
            let path = ctx.static_files_path.join(kind).join(name);
            if !path.is_file() {
                Err(
                    AnsernoError::NotFound(path.to_str().unwrap_or_default().to_string())
                        .with_context(&ctx),
                )
            } else {
                Ok(NamedFile::open(path)
                    .map_err(|err| AnsernoError::from(err).with_context(&ctx))?
                    .into_response(&req))
            }
        }
        _ => Err(AnsernoError::NotAllowed("".to_string()).with_context(&ctx)),
    }
}

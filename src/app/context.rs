use derive_builder::Builder;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use tera::Tera;

use crate::{error::AnsernoError, library::CalibreLibrary};

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "AnsernoError"))]
pub struct Context {
    pub conn: DatabaseConnection,
    pub library: CalibreLibrary,
    pub template_engine: Tera,
    pub static_files_path: PathBuf,
}

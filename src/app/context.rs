use crate::error::AnsernoError;
use derive_builder::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "AnsernoError"))]
pub struct Context {
    pub database_connection: sea_orm::DatabaseConnection,
    pub template_engine: tera::Tera,
    pub static_files_directory: std::path::PathBuf,
    pub library: crate::library::CalibreLibrary,

    #[cfg(feature = "search")]
    #[builder(default = "None")]
    pub search_index: Option<crate::library::SearchIndex>,
}

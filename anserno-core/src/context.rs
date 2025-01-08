use std::path::{Path, PathBuf};

#[derive(::std::clone::Clone, ::std::fmt::Debug, derive_builder::Builder)]
pub struct Context {
    library: calibre_data::library::RemoteLibrary,
    template_engine: tera::Tera,

    #[builder(setter(into))]
    static_files_dir: PathBuf,
}

impl Context {
    pub fn new(
        library: calibre_data::library::RemoteLibrary,
        template_engine: tera::Tera,
        static_files_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            library,
            template_engine,
            static_files_dir: static_files_dir.into(),
        }
    }

    #[inline]
    pub fn template_engine(&self) -> &tera::Tera {
        &self.template_engine
    }

    #[inline]
    pub fn library(&self) -> &calibre_data::library::RemoteLibrary {
        &self.library
    }

    #[inline]
    pub fn static_files_dir(&self) -> &Path {
        &self.static_files_dir
    }
}

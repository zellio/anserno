use std::path::{Path, PathBuf};

use crate::{entities::flat_books, error::Result};

pub trait LibraryResource {
    fn path(&self) -> Option<PathBuf>;
}

/// Calibre Library trait.
pub trait CalibreLibrary {
    type ResourcePath;

    /// Path to the library folder
    fn path(&self) -> &Path;

    /// Path to the library metadata database
    fn database(&self) -> PathBuf;

    fn database_path(&self) -> Option<Self::ResourcePath>;

    /// Connect to the metadata database
    fn connect(
        &mut self,
    ) -> impl ::std::future::Future<Output = Result<&sea_orm::DatabaseConnection>> {
        self.connect_with_config(|_| {})
    }

    /// Connect to the metadata database with control over the connection options
    fn connect_with_config<C>(
        &mut self,
        configurer: C,
    ) -> impl ::std::future::Future<Output = Result<&sea_orm::DatabaseConnection>>
    where
        C: FnMut(&mut sea_orm::ConnectOptions);

    /// Current database connection
    fn conn(&self) -> &sea_orm::DatabaseConnection;

    /// Internal file path of a given resource
    fn resource_path(&self, resource_name: &str) -> Result<Self::ResourcePath>;

    /// Internal file path for a given flat_book's resource
    fn flat_book_resource_path(
        &self,
        flat_book: &flat_books::Model,
        resource_name: &str,
    ) -> Result<Self::ResourcePath>;
}

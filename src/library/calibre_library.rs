use crate::{entities::books, queries::FlatBook};
use anyhow::Result;
use sea_orm::DbErr;
use std::path::PathBuf;
use url::{Origin, Url};

#[derive(Debug, Clone)]
pub struct CalibreLibrary {
    url: Option<String>,
    path: PathBuf,
}

impl CalibreLibrary {
    pub fn new(source: Url) -> Self {
        match source.origin() {
            Origin::Opaque(_) => Self {
                url: None,
                path: source.to_file_path().unwrap(),
            },

            ref origin @ Origin::Tuple(_, _, _) => Self {
                url: Some(origin.unicode_serialization()),
                path: PathBuf::from(source.path()),
            },
        }
    }

    pub fn is_local(&self) -> bool {
        self.url.is_none()
    }

    pub fn fpath(&self, fname: &str) -> String {
        self.path.join(fname).to_str().unwrap().to_string()
    }

    pub fn path(&self, fname: &str) -> String {
        let mut path = self.url.clone().unwrap_or("".to_string());
        path.push_str(self.fpath(fname).as_str());
        path
    }

    pub fn database_fpath(&self) -> String {
        self.fpath("metadata.db")
    }

    pub fn database_path(&self) -> String {
        self.path("metadata.db")
    }

    pub fn book_fpath(&self, book: &books::Model, fpath: &str) -> String {
        let path = PathBuf::from(book.path.as_str()).join(fpath);
        self.fpath(path.to_str().unwrap())
    }

    pub fn book_path(&self, book: &books::Model, fpath: &str) -> String {
        let path = PathBuf::from(book.path.as_str()).join(fpath);
        self.path(path.to_str().unwrap())
    }

    pub fn flat_book_fpath(&self, book: &FlatBook, fpath: &str) -> String {
        let path = PathBuf::from(book.path.as_str()).join(fpath);
        self.fpath(path.to_str().unwrap())
    }

    pub fn flat_book_path(&self, book: &FlatBook, fpath: &str) -> String {
        let path = PathBuf::from(book.path.as_str()).join(fpath);
        self.path(path.to_str().unwrap())
    }

    pub fn flat_book_format_fname(
        &self,
        flat_book: &FlatBook,
        format: &str,
    ) -> Result<String, DbErr> {
        let formats = flat_book
            .formats
            .as_object()
            .map(serde_json::Map::to_owned)
            .unwrap_or_else(|| {
                let mut map = serde_json::Map::new();
                map.extend(vec![("EPUB".to_string(), serde_json::Value::from(1))]);
                map
            });

        tracing::error!("{:?}", formats);

        formats
            .get(&format.to_uppercase())
            .and_then(serde_json::Value::as_str)
            .map(|format_filename| format!("{format_filename}.{}", format.to_lowercase()))
            .ok_or(DbErr::RecordNotFound(format!(
                "No matching format found for FlatBook(id: {}, format: {})",
                flat_book.id, format,
            )))
    }
}

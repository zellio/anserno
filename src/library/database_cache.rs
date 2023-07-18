use std::{fs::File, io::Write, path::PathBuf};

use crate::error::{AnsernoError, AnsernoResult};

use super::CalibreLibrary;

#[derive(Debug)]
pub struct DatabaseCache<'a> {
    library: &'a CalibreLibrary,
    temp_dir: tempfile::TempDir,
}

impl<'a> DatabaseCache<'a> {
    pub fn new(library: &'a CalibreLibrary) -> DatabaseCache {
        DatabaseCache {
            library,
            temp_dir: tempfile::Builder::new()
                .prefix(&format!("{}-", env!("CARGO_PKG_NAME")))
                .rand_bytes(8)
                .tempdir()
                .unwrap(),
        }
    }

    pub fn filepath(&self) -> PathBuf {
        self.temp_dir.path().join("metadata.db")
    }

    pub fn database_url(&self) -> Option<String> {
        let filepath = self.filepath();
        filepath.to_str().and_then(|path| {
            if filepath.exists() {
                Some(format!("sqlite://{path}"))
            } else {
                None
            }
        })
    }

    pub async fn fetch_database(&self) -> AnsernoResult<usize> {
        let source = self.library.database_path();

        if self.library.is_local() {
            std::fs::copy(source, self.filepath()).map(|e| e as usize)
        } else {
            let bytes = reqwest::get(source).await?.bytes().await?;
            let mut target = File::create(self.filepath())?;
            target.write(&bytes)
        }
        .map_err(AnsernoError::from)
    }
}

impl<'a> From<&'a CalibreLibrary> for DatabaseCache<'a> {
    fn from(value: &'a CalibreLibrary) -> Self {
        DatabaseCache::new(value)
    }
}

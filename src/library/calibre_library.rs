use sea_orm::DatabaseConnection;
use std::{fs::File, io::Write, path::PathBuf, sync::Arc};
use tempfile::{Builder as TempFileBuilder, TempDir};
use url::{Origin, Url};

use crate::{
    entities::flat_books,
    error::AnsernoResult,
    queries::{AnsernoQuery, FlatBooks, SearchIndex},
};

#[derive(Clone, Debug)]
pub struct CalibreLibrary {
    source: Url,
    tempdir: Arc<TempDir>,
}

impl TryFrom<Url> for CalibreLibrary {
    type Error = std::io::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        Ok(Self {
            source: value,
            tempdir: Arc::from(
                TempFileBuilder::new()
                    .prefix(&format!("{}-", env!("CARGO_PKG_NAME")))
                    .rand_bytes(8)
                    .tempdir()?,
            ),
        })
    }
}

impl CalibreLibrary {
    fn database_path(&self) -> PathBuf {
        self.tempdir.path().join("metadata.db")
    }

    pub fn database_url(&self) -> Option<String> {
        self.database_path().to_str().and_then(|path| {
            self.database_path()
                .exists()
                .then(|| format!("sqlite://{path}"))
        })
    }

    pub async fn fetch(&self) -> AnsernoResult<usize> {
        match self.source.origin() {
            Origin::Opaque(_) => Ok(std::fs::copy(
                self.source.to_file_path()?.join("metadata.db"),
                self.database_path(),
            )
            .map(|count| count as usize)?),

            Origin::Tuple(_, _, _) => {
                let mut source = self.source.clone();
                source.set_path("metadata.db");

                let bytes = reqwest::get(source).await?.bytes().await?;

                let mut target = File::create(self.database_path())?;
                Ok(target.write(&bytes)?)
            }
        }
    }

    pub async fn setup(&self, conn: &DatabaseConnection) -> AnsernoResult<()> {
        FlatBooks::execute(conn).await?;
        SearchIndex::execute(conn).await?;

        Ok(())
    }

    pub fn flat_book_file_url(
        &self,
        flat_book: flat_books::Model,
        filename: &str,
    ) -> AnsernoResult<Url> {
        let mut file_url = self.source.clone();
        {
            let mut path_segments = file_url.path_segments_mut()?;
            path_segments.extend(vec![flat_book.path.as_str(), filename]);
        }
        Ok(file_url)
    }
}

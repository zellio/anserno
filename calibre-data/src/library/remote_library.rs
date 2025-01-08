use futures_util::StreamExt;
use std::{io::Write, path::Path};

pub const REMOTE_LIBRARY_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

use crate::{
    entities::flat_books,
    error::{Error, Result},
    library::CalibreLibrary,
    queries::{CreateFlatBooksView, CreateSearchIndex, PopulateSearchIndex, StaticQuery},
};

#[derive(::std::fmt::Debug, ::std::clone::Clone)]
pub struct RemoteLibrary {
    tempdir: ::std::sync::Arc<tempfile::TempDir>,
    source: url::Url,
    conn: Option<sea_orm::DatabaseConnection>,
}

impl RemoteLibrary {
    /// Create a new remote library from source url.
    pub fn new(source: url::Url) -> Result<Self> {
        Ok(Self {
            tempdir: std::sync::Arc::new(tempfile::TempDir::new()?),
            source,
            conn: None,
        })
    }

    /// Copy the Calibre database to a temporary directory. Handles both local
    /// databases via `file://` and remote libraries via `http(s)://`.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn fetch_database(&self) -> Result<usize> {
        match self.source.origin() {
            url::Origin::Opaque(_) => {
                let source_path = self
                    .source
                    .to_file_path()
                    .map_err(|_| {
                        Error::RemoteLibrary(format!(
                            "Failed rendering database source: {:?}",
                            self.source
                        ))
                    })?
                    .join("metadata.db");

                ::std::fs::copy(source_path, self.database())
                    .map(|count| count as usize)
                    .map_err(Error::from)
            }

            url::Origin::Tuple(_, _, _) => {
                let mut source_url = self.source.clone();
                source_url
                    .path_segments_mut()
                    .map_err(|_| {
                        Error::RemoteLibrary(format!(
                            "Failed fetching mutable segments of database source: {:?}",
                            self.source
                        ))
                    })?
                    .push("metadata.db");

                let client = reqwest::ClientBuilder::default()
                    .user_agent(REMOTE_LIBRARY_USER_AGENT)
                    .build()?;

                let mut target = ::std::fs::File::create(self.database())?;
                let mut bytes_stream = client.get(source_url).send().await?.bytes_stream();

                let mut total_written = 0;

                while let Some(chunk) = bytes_stream.next().await.transpose()? {
                    target.write_all(&chunk)?;
                    total_written += chunk.len();
                }

                Ok(total_written)
            }
        }
    }
}

impl CalibreLibrary for RemoteLibrary {
    type ResourcePath = url::Url;

    #[inline]
    fn path(&self) -> &Path {
        self.tempdir.path()
    }

    #[inline]
    fn database(&self) -> std::path::PathBuf {
        self.path().join("metadata.db")
    }

    fn database_path(&self) -> Option<Self::ResourcePath> {
        self.database().to_str().map(|path| {
            let mut url = self.source.clone();
            url.set_path(path);
            url
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(configurer)))]
    async fn connect_with_config<C>(
        &mut self,
        mut configurer: C,
    ) -> Result<&sea_orm::DatabaseConnection>
    where
        C: FnMut(&mut sea_orm::ConnectOptions),
    {
        self.fetch_database().await?;

        let mut url = url::Url::parse("sqlite:///dummy")?;
        url.set_path(
            self.database().as_path().to_str().ok_or_else(|| {
                Error::RemoteLibrary("Failed rendering database path".to_string())
            })?,
        );

        let mut opts = sea_orm::ConnectOptions::new(url.as_str());

        configurer(&mut opts);

        self.conn = Some(sea_orm::Database::connect(opts).await?);

        CreateFlatBooksView::execute(self.conn()).await?;
        CreateSearchIndex::execute(self.conn()).await?;
        PopulateSearchIndex::execute(self.conn()).await?;

        Ok(self.conn.as_ref().unwrap())
    }

    #[inline]
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.conn.as_ref().unwrap()
    }

    fn resource_path(&self, resource_name: &str) -> Result<Self::ResourcePath> {
        let mut resource_url = self.source.clone();

        {
            let mut segments = resource_url.path_segments_mut().map_err(|_| {
                Error::RemoteLibrary("Failed fetching resource path segments mut".to_string())
            })?;
            segments.push(resource_name);
        };

        Ok(resource_url)
    }

    #[inline]
    fn flat_book_resource_path(
        &self,
        flat_book: &flat_books::Model,
        resource_name: &str,
    ) -> Result<Self::ResourcePath> {
        let mut resource_url = self.source.clone();
        {
            let mut segments = resource_url.path_segments_mut().map_err(|_| {
                Error::RemoteLibrary("Failed fetching resource path segments mut".to_string())
            })?;
            segments.extend([&flat_book.path, resource_name]);
        };
        Ok(resource_url)
    }
}

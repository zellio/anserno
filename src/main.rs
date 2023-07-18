use actix_web::{middleware, web, App, HttpServer};
use anserno::{
    app,
    error::{AnsernoError, AnsernoResult},
    library::{CalibreLibrary, DatabaseCache},
};

#[cfg(feature = "search")]
use anserno::library::SearchIndex;

use clap::Parser;
use sea_orm::{ConnectOptions, Database};
use std::path::PathBuf;
use tera::Tera;
use tracing::log;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{filter, prelude::*, Registry};

#[derive(clap::ValueEnum, Clone, Debug)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<&LogLevel> for filter::LevelFilter {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Trace => Self::TRACE,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
            LogLevel::Off => Self::OFF,
        }
    }
}

impl From<&LogLevel> for log::LevelFilter {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Trace => Self::Trace,
            LogLevel::Debug => Self::Debug,
            LogLevel::Info => Self::Info,
            LogLevel::Warn => Self::Warn,
            LogLevel::Error => Self::Error,
            LogLevel::Off => Self::Off,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum LogFormat {
    Auto,
    Plain,
    Pretty,
    Json,
}

/// A light, opinionated web view for Calibre Libraries.
#[derive(clap::Parser, Debug)]
#[clap(author, version)]
pub struct CliArgs {
    /// Log level
    #[clap(
        long,
        value_enum,
        default_value_t = LogLevel::Info,
        env("ANSERNO_LOG_LEVEL"),
    )]
    log_level: LogLevel,

    /// Log format
    #[clap(
        long,
        value_enum,
        default_value_t = LogFormat::Auto,
        env("ANSERNO_LOG_FORMAT"),
    )]
    log_format: LogFormat,

    ///
    #[clap(long, default_value = "127.0.0.1", env = "ANSERNO_HOST")]
    host: String,

    ///
    #[clap(long, default_value_t = 8080, env("ANSERNO_PORT"))]
    port: i16,

    ///
    #[clap(
        long,
        default_value = "templates/**/*.html",
        env("ANSERNO_TEMPLATES_DIRECTORY")
    )]
    templates_directory: String,

    ///
    #[clap(long, default_value = "static", env("ANSERNO_STATIC_FILES_DIRECTORY"))]
    static_files_directory: PathBuf,

    ///
    #[clap(long, value_enum, default_value_t = LogLevel::Debug, env("ANSERNO_SQLX_LOG_LEVEL"))]
    anserno_sqlx_log_level: LogLevel,

    ///
    #[clap(long, env("ANSERNO_LIBRARY_URL"))]
    library_url: url::Url,
}

#[actix_web::main]
async fn main() -> AnsernoResult<()> {
    let cli_args = CliArgs::parse();

    let trace_formatter = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let registry = Registry::default()
        .with::<filter::LevelFilter>(Into::<filter::LevelFilter>::into(&cli_args.log_level));

    match cli_args.log_format {
        LogFormat::Json => registry.with(trace_formatter.json()).init(),
        LogFormat::Pretty => registry.with(trace_formatter.pretty()).init(),
        _ => registry.with(trace_formatter.compact()).init(),
    };

    let library = CalibreLibrary::new(cli_args.library_url);

    let database_cache = DatabaseCache::new(&library);

    database_cache.fetch_database().await?;

    let url = database_cache
        .database_url()
        .ok_or(AnsernoError::Unknown("Bad database url".to_string()))?;

    let mut db_opts = ConnectOptions::new(url);
    db_opts
        .sqlx_logging(true)
        .sqlx_logging_level(Into::<log::LevelFilter>::into(
            &cli_args.anserno_sqlx_log_level,
        ));

    let mut app_ctx = app::ContextBuilder::default()
        .library(library)
        .database_connection(Database::connect(db_opts).await?)
        .template_engine(Tera::new(&cli_args.templates_directory)?)
        .static_files_directory(cli_args.static_files_directory)
        .build()?;

    #[cfg(feature = "search")]
    {
        tracing::info!("Building search index");

        let search_index = SearchIndex::default();
        search_index.drop_search_index(&app_ctx).await?;
        search_index.create_search_index(&app_ctx).await?;
        search_index.insert_search_index(&app_ctx).await?;

        app_ctx.search_index = Some(search_index)
    }

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(app_ctx.clone()))
            .configure(app::configure_app)
    })
    .bind(format!("{}:{}", &cli_args.host, &cli_args.port))
    .unwrap()
    .run()
    .await
    .map_err(AnsernoError::from)
}

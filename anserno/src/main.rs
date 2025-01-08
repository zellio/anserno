use actix_web::{middleware, web, App, HttpServer};
use anserno::logging::LogFormat;
use anserno_core::{config, context::ContextBuilder};
use calibre_data::library::{CalibreLibrary, RemoteLibrary};
use clap::Parser;
use tera::Tera;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Registry};

#[tokio::main]
pub async fn main() -> anserno::error::Result<()> {
    let args = anserno::cli::Args::parse();

    let trace_formatter = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let registry = Registry::default()
        .with::<filter::LevelFilter>(Into::<filter::LevelFilter>::into(args.log_level));

    match args.log_format {
        LogFormat::Json => registry.with(trace_formatter.json()).init(),
        LogFormat::Pretty => registry.with(trace_formatter.pretty()).init(),
        _ => registry.with(trace_formatter.compact()).init(),
    };

    tracing::info!("Setting up web execution context");

    let mut library = RemoteLibrary::new(args.library_url)?;
    library
        .connect_with_config(|config| {
            config
                .sqlx_logging(true)
                .sqlx_logging_level(args.sqlx_log_level.into());
        })
        .await?;

    let context = ContextBuilder::default()
        .library(library)
        .template_engine(Tera::new(&args.templates_glob).unwrap())
        .static_files_dir(&args.static_files_dir)
        .build()
        .unwrap();

    tracing::info!(
        "Starting anserno web server on {}:{}",
        &args.host,
        &args.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(context.clone()))
            .configure(config::configure)
    })
    .bind(format!("{}:{}", &args.host, &args.port))?
    .run()
    .await?;

    Ok(())
}

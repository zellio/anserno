use crate::logging;

#[derive(clap::Parser)]
#[command(author, version)]
pub struct Args {
    /// Set log level
    #[clap(long, value_enum, default_value_t = logging::LogLevel::Info, env("ANSERNO_LOG_LEVEL"))]
    pub log_level: logging::LogLevel,

    /// Set database query log level
    #[clap(long, value_enum, default_value_t = logging::LogLevel::Debug, env("ANSERNO_SQLX_LOG_LEVEL"))]
    pub sqlx_log_level: logging::LogLevel,

    /// Set log format
    #[clap(long, value_enum, default_value_t = logging::LogFormat::Auto, env("ANSERNO_LOG_FORMAT"))]
    pub log_format: logging::LogFormat,

    /// Web server bind host
    #[clap(long, default_value = "127.0.0.1", env = "ANSERNO_HOST")]
    pub host: String,

    /// Web server bind port
    #[clap(long, default_value_t = 8080, env("ANSERNO_PORT"))]
    pub port: i16,

    /// Source url for Calibre library (can be a file:// url for local library)
    #[clap(long, env("ANSERNO_LIBRARY_URL"))]
    pub library_url: url::Url,

    /// Path to the anserno-core templates for html rendering
    #[clap(
        long,
        default_value = "anserno-core/templates/**/*.html",
        env("ANSERNO_TEMPLATES_GLOB")
    )]
    pub templates_glob: String,

    /// Path to the anserno-core static files (javascript / css)
    #[clap(
        long,
        default_value = "anserno-core/static",
        env("ANSERNO_STATIC_FILES_PATH")
    )]
    pub static_files_dir: std::path::PathBuf,
}

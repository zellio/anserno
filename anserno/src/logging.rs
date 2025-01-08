/// Log level adapter for clap cli parser
#[derive(::core::marker::Copy, ::std::clone::Clone, ::std::fmt::Debug, clap::ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<&LogLevel> for tracing_subscriber::filter::LevelFilter {
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

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(value: LogLevel) -> Self {
        tracing_subscriber::filter::LevelFilter::from(&value)
    }
}

impl From<&LogLevel> for tracing::log::LevelFilter {
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

impl From<LogLevel> for tracing::log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        tracing::log::LevelFilter::from(&value)
    }
}

#[derive(::core::marker::Copy, ::std::clone::Clone, ::std::fmt::Debug, clap::ValueEnum)]
pub enum LogFormat {
    Auto,
    Plain,
    Pretty,
    Json,
}

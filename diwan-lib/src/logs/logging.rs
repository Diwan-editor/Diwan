use anyhow::{Context, Error, Result};
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::{Color, ConfigBuilder, WriteLogger};
use std::{
    env,
    fmt::format,
    fs::{create_dir_all, File, OpenOptions},
    path::PathBuf,
};
use time::{macros::format_description, UtcOffset};

/// A custom logger implementation for the Diwan application that handles file-based logging
/// with configurable log levels and formatted output.
#[derive(Debug, PartialEq)]
pub struct DiwanLogger {
    /// The path to the log file where messages will be written
    pub file: PathBuf,
    /// The maximum log level filter that determines which messages are recorded
    pub level: LevelFilter,
}

/// Represents the available logging levels in the Diwan application.
/// These levels map to the standard log crate's levels but with simplified naming.
#[derive(Debug, Clone)]
pub enum DiwanLevelLog {
    /// Detailed information for debugging purposes
    Debug,
    /// General information about program execution
    Info,
    /// Potentially harmful situations
    Warn,
    /// Critical errors that may cause program failure
    Critical,
    /// trace all errors
    Trace,
}

// TODO: write docs here
// TODO: add a bunch of cfgs to check if the log flag is activated , maybe diwan can be run in some low
// privilege that wont let it create log files !

impl DiwanLogger {
    /// Creates a new `DiwanLogger` instance that stores log files in the system's cache directory.
    ///
    /// # Arguments
    ///
    /// * `levellog` - The desired logging level for the logger instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `DiwanLogger` or an `Error` if the cache directory could not be determined
    /// or created.
    pub fn new(levellog: DiwanLevelLog) -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .context("Failed to determine cache directory")?
            .join("diwan");

        Self::create_logger(levellog, cache_dir.join("di.log"))
    }

    /// Creates a new `DiwanLogger` instance that stores log files in the current working directory.
    ///
    /// This can be used for development or debugging sessions where file access is restricted to the local project folder.
    ///
    /// # Arguments
    ///
    /// * `levellog` - The desired logging level for the logger instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `DiwanLogger` or an `Error` if the current directory could not be determined.
    pub fn new_local(levellog: DiwanLevelLog) -> Result<Self> {
        let current_dir = env::current_dir().context("Failed to determine current directory")?;

        Self::create_logger(levellog, current_dir.join("di.log"))
    }

    /// Initializes the logger with the provided logging level and log file path.
    ///
    /// This internal helper function is shared by both `new` and `new_local` constructors.
    ///
    /// # Arguments
    ///
    /// * `levellog` - The desired logging level.
    /// * `log_path` - The full path where the log file should be stored.
    ///
    /// # Returns
    ///
    /// A `Result` with a configured `DiwanLogger` or an error if the log directory could not be created.
    #[inline]
    fn create_logger(levellog: DiwanLevelLog, log_path: PathBuf) -> Result<Self> {
        if let Some(parent) = log_path.parent() {
            create_dir_all(parent).context("Failed to create log directory")?;
        }

        Ok(Self {
            file: log_path,
            level: Self::map_level(levellog),
        })
    }

    /// Maps the custom `DiwanLevelLog` enum to the standard `LevelFilter` used by the `log` crate.
    ///
    /// # Arguments
    ///
    /// * `level` - The custom log level to map.
    ///
    /// # Returns
    ///
    /// A `LevelFilter` representing the equivalent standard log level.
    #[inline]
    fn map_level(level: DiwanLevelLog) -> LevelFilter {
        match level {
            DiwanLevelLog::Debug => LevelFilter::Debug,
            DiwanLevelLog::Info => LevelFilter::Info,
            DiwanLevelLog::Warn => LevelFilter::Warn,
            DiwanLevelLog::Critical => LevelFilter::Error,
            DiwanLevelLog::Trace => LevelFilter::Trace,
        }
    }

    /// Configures and initializes the logging system using `simplelog` with color and formatting options.
    ///
    /// This sets up the logger to write to the file specified in the `DiwanLogger` instance, with time stamps,
    /// color-coded levels, and filtering for Diwan-specific logs.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of logger initialization.
    pub fn setup_dn_logger(&self) -> Result<()> {
        let config = ConfigBuilder::new()
            .add_filter_allow_str("diwan")
            .add_filter_allow_str("dn")
            .set_thread_level(LevelFilter::Error)
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .set_level_color(log::Level::Error, Some(Color::Red))
            .set_level_color(log::Level::Warn, Some(Color::Yellow))
            .set_level_color(log::Level::Info, Some(Color::Green))
            .set_level_color(log::Level::Debug, Some(Color::Blue))
            .set_time_format_custom(format_description!(
                "[day]-[month]-[year] [hour repr:12]:[minute]:[second]"
            ))
            .set_time_offset(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC))
            .build();

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file)
            .context(format!("Failed to open log file: {:?}", self.file))?;

        WriteLogger::init(self.level, config, log_file).context("Logger initialization failed")?;

        Ok(())
    }

    /// Writes a single log message to the configured logger at the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The logging level at which the message should be recorded.
    /// * `message` - The log message to write.
    pub fn write_to_dn_log(&self, level: DiwanLevelLog, message: &str) {
        match level {
            DiwanLevelLog::Debug => debug!("{}", message),
            DiwanLevelLog::Info => info!("{}", message),
            DiwanLevelLog::Warn => warn!("{}", message),
            DiwanLevelLog::Critical => error!("{}", message),
            DiwanLevelLog::Trace => trace!("{}", message),
        }
    }
}

use anyhow::{Context, Error, Result};
use log::{debug, error, info, warn, LevelFilter};
use simplelog::{
    Color, ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{
    env,
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
};
use time::{macros::format_description, OffsetDateTime, UtcOffset};

/// A custom logger implementation for the Diwan application that handles file-based logging
/// with configurable log levels and formatted output.
pub struct DiwanLogger {
    /// The path to the log file where messages will be written
    pub file: PathBuf,
    /// The maximum log level filter that determines which messages are recorded
    pub level: LevelFilter,
}

/// Represents the available logging levels in the Diwan application.
/// These levels map to the standard log crate's levels but with simplified naming.
#[derive(Debug)]
pub enum DiwanLevelLog {
    /// Detailed information for debugging purposes
    Debug,
    /// General information about program execution
    Info,
    /// Potentially harmful situations
    Warn,
    /// Critical errors that may cause program failure
    Critical,
}

// TODO: write docs here
// TODO: add a bunch of cfgs to check if the log flag is activated , maybe diwan can be run in some low
// privilege that wont let it create log files !

impl DiwanLogger {
    /// Creates a new DiwanLogger instance with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `levellog` - The desired logging level for the logger
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - A new logger instance or an error if initialization fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The HOME environment variable is not set
    /// * The log directory cannot be created
    /// * The parent directory path is invalid
    pub fn new(levellog: DiwanLevelLog) -> Result<Self, Error> {
        // TODO : to remove this , home_dir can be infered from the pwd
        let home_dir = env::var("HOME").context("Couldn't retrieve HOME environment variable")?;
        let diwan_log_path = PathBuf::from(format!("{}/.cache/diwan/diwan.log", home_dir));

        // TODO : A probing operation must be executed to check if "Diwan" is allowed to make dirs and files in the
        // purported path
        if let Some(parent_dir) = diwan_log_path.parent() {
            create_dir_all(parent_dir).with_context(|| {
                format!(
                    "Failed to create or access directory: {}",
                    parent_dir.display()
                )
            })?;
        } else {
            return Err(anyhow::anyhow!(
                "Invalid path: could not determine parent directory for {}",
                diwan_log_path.display()
            ));
        }
        let level = match levellog {
            DiwanLevelLog::Debug => LevelFilter::Debug,
            DiwanLevelLog::Info => LevelFilter::Info,
            DiwanLevelLog::Warn => LevelFilter::Warn,
            DiwanLevelLog::Critical => LevelFilter::Error,
        };

        Ok(Self {
            file: diwan_log_path,
            level,
        })
    }

    /// Initializes the logging system with custom configuration and file output.
    ///
    /// Sets up a configured logger with:
    /// * Custom time format
    /// * Color-coded output for different log levels
    /// * Local timezone support
    /// * Filtered logging for "diwan" and "dn" modules
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or an error if logger initialization fails
    pub fn setup_dn_logger(&self) -> Result<(), Error> {
        let local_offset = Self::get_local_time()?;
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
            .set_time_offset(local_offset)
            .build();

        let log_file = self.create_log_file()?;

        WriteLogger::init(self.level, config, log_file).context("Failed to initialize logger")?;

        Ok(())
    }

    /// Writes a message to the log file with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level to use for the message
    /// * `message` - The message to write to the log
    pub fn write_to_dn_log(&self, level: DiwanLevelLog, message: &str) {
        match level {
            DiwanLevelLog::Debug => debug!("{}", message),
            DiwanLevelLog::Info => info!("{}", message),
            DiwanLevelLog::Warn => warn!("{}", message),
            DiwanLevelLog::Critical => error!("{}", message),
        }
    }

    /// Creates or opens the log file in append mode.
    ///
    /// # Returns
    ///
    /// * `Result<File, Error>` - The opened file handle or an error if the operation fails
    fn create_log_file(&self) -> Result<File, Error> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file)
            .context(format!(
                "Failed to create or open log file at {:?}",
                self.file
            ))
    }

    /// Attempts to get the local timezone offset, falling back to UTC if unsuccessful.
    ///
    /// # Returns
    ///
    /// * `Result<UtcOffset, Error>` - The local timezone offset or UTC if local offset cannot be determined
    fn get_local_time() -> Result<UtcOffset, Error> {
        UtcOffset::current_local_offset().or_else(|_| {
            warn!("Local timezone offset could not be determined. Falling back to UTC.");
            Ok(UtcOffset::UTC)
        })
    }
}

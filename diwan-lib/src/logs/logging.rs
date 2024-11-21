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
use time::{macros::format_description, UtcOffset};

pub struct DiwanLogger {
    pub file: PathBuf,
    pub level: LevelFilter,
}

#[derive(Debug)]
pub enum DiwanLevelLog {
    All,
    Debug,
    Info,
    Warn,
    Critical,
}

impl DiwanLogger {
    /// Create a new `DiwanLogger` instance
    pub fn new(levellog: DiwanLevelLog) -> Result<Self, Error> {
        // Retrieve the HOME directory
        let home_dir = env::var("HOME").context("Couldn't retrieve HOME environment variable")?;
        let diwan_log_path = PathBuf::from(format!("{}/.cache/diwan/diwan.log", home_dir));

        // Map the logging level
        let level = match levellog {
            DiwanLevelLog::Debug => LevelFilter::Debug,
            DiwanLevelLog::Info => LevelFilter::Info,
            DiwanLevelLog::Warn => LevelFilter::Warn,
            DiwanLevelLog::Critical => LevelFilter::Error,
            DiwanLevelLog::All => LevelFilter::Trace,
        };

        Ok(Self {
            file: diwan_log_path,
            level,
        })
    }

    /// Set up the logger
    pub fn setup_dn_logger(&self) -> Result<(), Error> {
        // Define a custom configuration for the logger
        let config = ConfigBuilder::new()
            .add_filter_allow_str("diwan")
            .add_filter_allow_str("dn")
            .set_thread_level(LevelFilter::Error)
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .set_time_format_custom(format_description!(
                "[month]-[day]-[year] [hour repr:12]:[minute]:[second] [period]"
            ))
            .set_time_offset(Self::get_local_time()?)
            .set_level_color(log::Level::Error, Some(Color::Red))
            .set_level_color(log::Level::Warn, Some(Color::Yellow))
            .set_level_color(log::Level::Info, Some(Color::Green))
            .set_level_color(log::Level::Debug, Some(Color::Blue))
            .build();

        // Always attempt to create the log directory and file
        let log_file = self.create_log_file()?;

        // Initialize the WriteLogger
        WriteLogger::init(self.level, config, log_file).context("Failed to initialize logger")?;

        Ok(())
    }

    /// Log a message at the appropriate level
    pub fn write_to_dn_log(&self, level: DiwanLevelLog, message: &str) {
        match level {
            DiwanLevelLog::Debug => debug!("{}", message),
            DiwanLevelLog::Info => info!("{}", message),
            DiwanLevelLog::Warn => warn!("{}", message),
            DiwanLevelLog::Critical => error!("{}", message),
            DiwanLevelLog::All => {
                todo!()
            }
        }
    }

    /// Create the log file (and its directory if necessary)
    fn create_log_file(&self) -> Result<File, Error> {
        if let Some(parent) = self.file.parent() {
            create_dir_all(parent).context("Failed to create log directory")?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file)
            .context(format!(
                "Failed to create or open log file at {:?}",
                self.file
            ))
    }

    /// Get the local timezone offset
    fn get_local_time() -> Result<UtcOffset, Error> {
        UtcOffset::current_local_offset().or_else(|_| {
            warn!("Local timezone offset could not be determined. Falling back to UTC.");
            Ok(UtcOffset::UTC)
        })
    }
}

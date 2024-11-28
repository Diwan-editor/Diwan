use anyhow::{Context, Error, Result as AnyhowResult};
use diwan::logs::{DiwanLevelLog, DiwanLogger};
use log::LevelFilter;
use std::{env, fs::read_to_string, path::PathBuf};

#[test]
fn test_diwan_logger() -> AnyhowResult<()> {
    // Setup test environment
    let home_dir = env::var("HOME").context("Couldn't retrieve HOME environment variable")?;
    let diwan_log_path = PathBuf::from(format!("{}/.cache/diwan/diwan.log", home_dir));

    // Test logger initialization with different levels
    let loggers = [
        (DiwanLevelLog::Debug, LevelFilter::Debug),
        (DiwanLevelLog::Info, LevelFilter::Info),
        (DiwanLevelLog::Warn, LevelFilter::Warn),
        (DiwanLevelLog::Critical, LevelFilter::Error),
    ];

    // Test logger setup and equality
    for (level, expected_filter) in loggers {
        let logger = DiwanLogger::new(level)?;
        assert_eq!(
            logger,
            DiwanLogger {
                file: diwan_log_path.clone(),
                level: expected_filter
            }
        );
    }

    // Test logger setup
    let debug_logger = DiwanLogger::new(DiwanLevelLog::Debug)?;
    debug_logger.setup_dn_logger()?;

    // Test writing logs
    let test_messages = [
        (DiwanLevelLog::Debug, "This is a test Debug message"),
        (DiwanLevelLog::Info, "This is a test Info message"),
        (DiwanLevelLog::Warn, "This is a test Warn message"),
        (DiwanLevelLog::Critical, "This is a test Critical message"),
    ];

    // Write test messages
    for (level, message) in &test_messages {
        debug_logger.write_to_dn_log(level.clone(), message);
    }

    // Verify log contents
    let log_content = read_to_string(&diwan_log_path)?;
    for (_, message) in &test_messages {
        assert!(
            log_content.contains(message),
            "Log file does not contain the message: {}",
            message
        );
    }

    // Optional: Test invalid path scenario
    // Commented out as it requires root privileges or special setup
    /*
    env::set_var("HOME", "/invalid_path");
    assert!(
        DiwanLogger::new(DiwanLevelLog::Debug).is_err(),
        "Logger creation should fail with invalid path"
    );
    */

    Ok(())
}

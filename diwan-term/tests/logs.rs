use anyhow::{Context, Error, Result as AnyhowResult};
use diwan::logs::{DiwanLevelLog, DiwanLogger};
use log::LevelFilter;
use std::{env, fs::read_to_string, path::PathBuf};

#[test]
/// simple function that test the diwan log setup
fn test_log_setup() -> AnyhowResult<(), Error> {
    let home_dir = env::var("HOME").context("Couldn't retrieve HOME environment variable")?;
    let diwan_log_path = PathBuf::from(format!("{}/.cache/diwan/diwan.log", home_dir));

    let diwan_lg = DiwanLogger::new(DiwanLevelLog::Debug)?;
    let diwan_lg1 = DiwanLogger::new(DiwanLevelLog::Info)?;
    let diwan_lg2 = DiwanLogger::new(DiwanLevelLog::Warn)?;
    let diwan_lg3 = DiwanLogger::new(DiwanLevelLog::Critical)?;
    diwan_lg.setup_dn_logger()?;

    assert_eq!(
        diwan_lg,
        DiwanLogger {
            file: diwan_log_path.clone(),
            level: LevelFilter::Debug
        }
    );
    assert_eq!(
        diwan_lg1,
        DiwanLogger {
            file: diwan_log_path.clone(),
            level: LevelFilter::Info
        }
    );
    assert_eq!(
        diwan_lg2,
        DiwanLogger {
            file: diwan_log_path.clone(),
            level: LevelFilter::Warn
        }
    );
    assert_eq!(
        diwan_lg3,
        DiwanLogger {
            file: diwan_log_path,
            level: LevelFilter::Error
        }
    );
    Ok(())
}

#[test]
// fn to test the log if it works
fn test_write_to_log() -> AnyhowResult<(), Error> {
    let home_dir = env::var("HOME").context("Couldn't retrieve HOME environment variable")?;
    let diwan_log_path = PathBuf::from(format!("{}/.cache/diwan/diwan.log", home_dir));
    let diwan_lg = DiwanLogger::new(DiwanLevelLog::Debug)?;

    let test_message = "This is a test Debug";
    diwan_lg.write_to_dn_log(DiwanLevelLog::Debug, test_message);
    let test_message1 = "This is a test Info";
    diwan_lg.write_to_dn_log(DiwanLevelLog::Info, test_message);
    // let test_message = "This is a test Warn";
    // diwan_lg.write_to_dn_log(DiwanLevelLog::Warn, test_message);
    // let test_message = "This is a test Error";
    // diwan_lg.write_to_dn_log(DiwanLevelLog::Critical, test_message);

    // content
    let log_content = read_to_string(diwan_log_path)?;
    assert!(
        log_content.contains(test_message),
        "Log file does not contain the expected message."
    );
    // let log_content1 = read_to_string(diwan_log_path.clone())?;
    // assert!(
    //     log_content1.contains(test_message1),
    //     "Log file does not contain the expected message."
    // );
    // let log_content2 = read_to_string(diwan_log_path.clone())?;
    // assert!(
    //     log_content2.contains(test_message2),
    //     "Log file does not contain the expected message."
    // );
    // let log_content3 = read_to_string(diwan_log_path)?;
    // assert!(
    //     log_content3.contains(test_message3),
    //     "Log file does not contain the expected message."
    // );
    Ok(())
}

// #[test]
// #[should_panic(expected = "Permission denied")]
// /// Test that logger setup fails for invalid directory
// fn failed_to_create_file_log() {
//     env::set_var("HOME", "/invalid_path");

//     let _ = DiwanLogger::new(DiwanLevelLog::Debug).unwrap();
// }

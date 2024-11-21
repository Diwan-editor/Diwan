pub mod loging {
    use std::{
        fs::File,
        io::{self, Write},
        time::SystemTime,
    };

    use env_logger::TimestampPrecision;
    pub struct Logger {
        pub file: File,
        pub last_message: String,
    }

    #[derive(Debug)]
    pub enum Criticality {
        Normal,
        Critical,
    }

    impl Logger {
        pub fn setup_login() -> Result<Logger, io::Error> {
            let f = File::options().append(true).open("log.txt")?;

            Ok(Self {
                file: f,
                last_message: "".to_string(),
            })
        }

        pub fn write_logs(
            &mut self,
            log_record: &str,
            criticality: Criticality,
        ) -> Result<(), io::Error> {
            self.file.write_fmt(format_args!(
                "\n [{:?}] -- message: {} -- {:#?}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()..,
                log_record,
                criticality
            ))?;

            Ok(())
        }
    }
}

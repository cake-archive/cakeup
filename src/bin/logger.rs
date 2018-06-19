use log::{self, Metadata, Log, Level, LevelFilter, Record, SetLoggerError};

const LOGGER: &'static Logger = &Logger(());

#[derive(Debug)]
pub struct Logger(());
impl Logger {
    pub fn init(trace: bool) -> Result<(), SetLoggerError> {
        let filter = if trace { LevelFilter::Trace } else { LevelFilter::Info };
        log::set_logger(LOGGER)
            .map(|()| log::set_max_level(filter))
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        return true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => {
                eprintln!("Error: {}", record.args());
            },
            Level::Warn => {
                println!("Warning: {}", record.args());
            },
            _ => {
                println!("{}", record.args());
            }
        }
    }

    fn flush(&self) {
    }
}
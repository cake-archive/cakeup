use log::{self, Log, Level, LevelFilter};

const LOGGER: &'static Logger = &Logger(());

#[derive(Debug)]
pub struct Logger(());
impl Logger {
    pub fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
    }
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        return true
    }

    fn log(&self, record: &log::Record) {
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
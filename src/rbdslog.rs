use chrono::{DateTime, Utc, FixedOffset};
use log::{info, trace, Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        let local_time: DateTime<Utc> = Utc::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        let china_timezone  = FixedOffset::west(16 * 3600);
        println!(
            "{} {} [{}] {}",
            utc_time.with_timezone(&china_timezone).format("%T"),
            record.level(),
            record.target(),
            record.args()
        );
    }
    fn flush(&self) {}
}

pub struct RBDSLogger;
impl RBDSLogger {
   pub fn init_log()  -> Result<(), SetLoggerError> {
        log::set_logger(&SimpleLogger).map(|()| log::set_max_level(LevelFilter::Info))
    }
}
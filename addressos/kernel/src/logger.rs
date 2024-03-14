use log::{Level, Log, Record};

use crate::{config::LOGGER, println};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level2color = |level: Level| match level {
            Level::Error => "31",
            Level::Warn => "33",
            Level::Info => "32",
            Level::Debug => "34",
            Level::Trace => "35",
        };

        (self.enabled(record.metadata())).then(|| {
            let color = level2color(record.level());

            println!(
                "\u{1B}[{}m[{:^5}] {}\u{1B}[0m",
                color,
                record.level(),
                record.args(),
            );
        });
    }

    fn flush(&self) {}
}

pub fn init() {
    let env2level = || match option_env!("LOG").unwrap_or("TRACE") {
        "TRACE" => log::LevelFilter::Trace,
        "DEBUG" => log::LevelFilter::Debug,
        "INFO" => log::LevelFilter::Info,
        "WARN" => log::LevelFilter::Warn,
        "ERROR" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(env2level());
}

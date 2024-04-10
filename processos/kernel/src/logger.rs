use log::{Level, LevelFilter, Log, Metadata, Record};
use spin::Once;

static LOGGER: Once<Logger> = Once::new();

pub struct Logger {
    pub color_enabled: bool,
}

impl Logger {
    fn level_to_color_code(&self, level: Level) -> &'static str {
        if self.color_enabled {
            match level {
                Level::Error => "31",
                Level::Warn => "33",
                Level::Info => "32",
                Level::Debug => "34",
                Level::Trace => "35",
            }
        } else {
            ""
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color_code = self.level_to_color_code(record.level());
            println!(
                "\u{1B}[{}m[{:^7}] {}\u{1B}[0m",
                color_code,
                record.level(),
                record.args(),
            );
        }
    }

    fn flush(&self) {}
}

pub fn init(color_enabled: bool) -> Result<(), log::SetLoggerError> {
    LOGGER.call_once(|| Logger { color_enabled });

    let logger = LOGGER.get().unwrap();

    log::set_logger(logger)?;

    let default_level = option_env!("LOG").unwrap_or("TRACE");
    let level_filter = default_level.parse().unwrap_or(LevelFilter::Trace);
    log::set_max_level(level_filter);

    Ok(())
}

use log::{Level, LevelFilter, Log, Metadata, Record};
use owo_colors::{AnsiColors, OwoColorize};

use crate::println;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => AnsiColors::BrightRed,
            Level::Warn => AnsiColors::BrightYellow,
            Level::Info => AnsiColors::BrightGreen,
            Level::Debug => AnsiColors::BrightBlue,
            Level::Trace => AnsiColors::BrightBlack,
        };

        println!(
            "[{}] {}: {}",
            record.level().color(color),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        Some("OFF") => LevelFilter::Off,
        _ => LevelFilter::Info,
    });
}

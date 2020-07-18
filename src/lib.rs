#![deny(warnings)]
#![no_std]

use log::{Log, LevelFilter};
use rtt_target::{rprintln, rtt_init_print};

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        rprintln!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

pub fn init() {
    rtt_init_print!();
    log::set_logger(&Logger).unwrap();
    if log::max_level() == LevelFilter::Off {
        log::set_max_level(LevelFilter::Info);
    }
}

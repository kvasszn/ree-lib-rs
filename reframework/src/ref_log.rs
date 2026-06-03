use crate::*;
use log::{Level, Metadata, Record, LevelFilter};

static PLUGIN_LOGGER: REFrameworkLogger = REFrameworkLogger;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{LazyLock, Mutex};

pub static LOG_FILE: LazyLock<Mutex<File>> = LazyLock::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ree_plugin.log")
        .expect("Failed to open log file");
    Mutex::new(file)
});

#[macro_export]
macro_rules! log_to_file {
    ($($arg:tt)*) => {
        if let Ok(mut file) = $crate::ref_log::LOG_FILE.lock() {
            use std::io::Write;
            let _ = writeln!(file, $($arg)*);
            let _ = file.flush();
        }
    };
}

/*
 * By default logs to a file and reframework
 * Logs only to a file using target: "file_only"
 * Logs only to ref using target: "ref_only"
 */
struct REFrameworkLogger;

impl log::Log for REFrameworkLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = format!("[{}] {}", record.level(), record.args());
        let target = record.target();

        if target != "ref_only" {
            if let Ok(mut file) = LOG_FILE.lock() {
                let _ = writeln!(file, "{}", msg);
            }
        }

        if target != "file_only" {
            match record.level() {
                Level::Error => log_error(&msg),
                Level::Warn => log_warn(&msg),
                _ => log_info(&msg), 
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = LOG_FILE.lock() {
            let _ = std::io::Write::flush(&mut *file);
        }
    }
}

pub fn initialize_logging() {
    log::set_logger(&PLUGIN_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .inspect_err(|e| log_to_file!("Failed to initialize standard logger {e}"))
        .expect("Failed to initialize standard logger");

    log::info!("REFramework Rust Logger initialized");
}

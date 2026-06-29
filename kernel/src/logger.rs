/*
 * A simple logger implementation that writes log messages to the serial port (COM1).
 * The log messages include a timestamp, log level, source file name, and line number.
 *
 * Author: Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-07
 * License: GPLv3
 */

use core::{fmt::Write, sync::atomic::{AtomicBool, Ordering::{self, Relaxed}}};
use log::{Metadata, Record};
use crate::device::{pit, serial::{self, COM1}};

/// A simple logger implementing the `log::Log` trait, writing to the serial port (COM1).
pub struct Logger {
    is_logging_to_terminal: AtomicBool,
}

impl Logger {
    /// Create a new logger.
    pub const fn new() -> Logger {
        Logger {
            is_logging_to_terminal: AtomicBool::new(false),
        }
    }

    pub fn enable_terminal_logging(&self, enabled: bool) {
        self.is_logging_to_terminal.store(enabled, Relaxed);
    }
}

impl log::Log for Logger {
    /// Check if the logger is enabled for the given metadata.
    /// This simple implementation always returns true.
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    /// Print a log record to the serial port.
    fn log(&self, record: &Record) {
        let level = level_abbreviation(record.metadata().level());
        //let target = record.metadata().target();
        let args = record.args();
        let file = record.file().unwrap_or("");
        let line = record.line().unwrap_or(0);
        //let path = record.module_path().unwrap_or("");

        if self.is_logging_to_terminal.load(Relaxed) {
            println!("[{}][{}][{}.{}] {}\n", pit::system_time(), level, file, line, args);
        }
        else {
            COM1.lock().write_fmt(format_args!("[{}][{}][{}.{}] {}\n", pit::system_time(), level, file, line, args)).unwrap();
        }
    }

    /// Flush the logger.
    /// Since all messages are written immediately, this is a no-op.
    fn flush(&self) {}
}

/// Convert a log level abbreviation to a `log::Level`.
/// Supported abbreviations are:
/// - "TRC" -> Trace
/// - "DBG" -> Debug
/// - "INF" -> Info
/// - "WRN" -> Warn
/// - "ERR" -> Error
/// Returns `None` for unrecognized abbreviations.
pub fn level_from_abbreviation(abbr: &str) -> Option<log::Level> {
    match abbr {
        "TRC" | "trc" => Some(log::Level::Trace),
        "DBG" | "dbg" => Some(log::Level::Debug),
        "INF" | "inf" => Some(log::Level::Info),
        "WRN" | "wrn" => Some(log::Level::Warn),
        "ERR" | "err" => Some(log::Level::Error),
        _ => None,
    }
}

/// Get the three-letter abbreviation for a given log level.
fn level_abbreviation(level: log::Level) -> &'static str {
    match level {
        log::Level::Trace => "TRC",
        log::Level::Debug => "DBG",
        log::Level::Info => "INF",
        log::Level::Warn => "WRN",
        log::Level::Error => "ERR",
    }
}
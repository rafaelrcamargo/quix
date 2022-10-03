//! # Loggy library
//! This library is used to log messages in a structured and beautiful way.
//!
//! ## About
//! This lib uses the `println` macro to log messages, exposes some macros to log messages in different levels.
//!
//! The levels are:
//! - **trace**: for debug messages.
//! - **debug**: for info messages.
//! - **success**: for success messages.
//! - **warn**: for warning messages.
//! - **error**: for error messages.
//! - **custom**: for custom messages.
//!
//! ## Examples
//! ```rust
//! #[macro_use] // We only need to import some macros to use `Loggy`.
//! extern crate loggy;
//!
//! // Let's start logging!
//! fn main() {
//!    // Info level.
//!    trace!("Starting the CLI.");   // 02:12:22 | 🔧 TRCE | Starting the CLI.
//!    debug!("Starting the CLI.");   // 02:12:22 | 🔮 DBUG | Starting the CLI.
//!
//!    // Confirmation level.
//!    success!("Starting the CLI."); // 02:12:22 | 🎉 YEEE | Starting the CLI.
//!
//!    // Warning & Error levels.
//!    warn!("Starting the CLI.");    // 02:12:22 | 💡 WARN | Starting the CLI.
//!    error!("Starting the CLI.");   // 02:12:22 | 💥 F#CK | Starting the CLI.
//!
//!    // Custom level. (Thats a different macro, here we define the level of the message as the way we want.)
//!    custom!("🧭 CSTM", format!("Starting the {}.", "CLI")); // 20:39:24 | 🧭 CSTM | Starting the CLI.
//! }
//! ```

mod interface;
pub use interface::Level;

mod functions;
pub use functions::*;

/// 💥 - Logs a message at the error level.
#[macro_export]
macro_rules! error {
    // error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Error, format_args!($($arg)+)))
}

/// 💡 - Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Warn, format_args!($($arg)+)))
}

/// 🔮 - Logs a message at the debug level.
#[macro_export]
macro_rules! debug {
    // debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Debug, format_args!($($arg)+)))
}

/// 📰 - Logs a message at the info level.
#[macro_export]
macro_rules! info {
    // info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Info, format_args!($($arg)+)))
}

/// 🎉 - Logs a message at the success level.
#[macro_export]
macro_rules! success {
    // success!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Success, format_args!($($arg)+)))
}

/// 🔧 - Logs a message at the trace level.
#[macro_export]
macro_rules! trace {
    // trace!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Trace, format_args!($($arg)+)))
}

/// 💭 - Logs a message at the help level.
#[macro_export]
macro_rules! help {
    // help!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Help, format_args!($($arg)+)))
}

/// 🧠 - Logs a message at the custom level.
#[macro_export]
macro_rules! custom {
    // custom!("a {} event", "log")
    ($level: expr, $message: expr) => {
        $crate::custom_log($level, $message)
    };
}

/// 📃 - Logs a JSON at the info level.
#[macro_export]
macro_rules! stringify {
    ($json: expr) => {
        $crate::json_log($json)
    };
}

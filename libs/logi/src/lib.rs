//! # Logi library
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
//! #[macro_use] // We only need to import some macros to use `Logi`.
//! extern crate logi;
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
//!    custom!("🧭 CSTM".to_string(), format!("Starting the {}.", "CLI")); // 20:39:24 | 🧭 CSTM | Starting the CLI.
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
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Error), format!($($arg)+)))
}

/// 💡 - Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Warn), format!($($arg)+)))
}

/// 🔮 - Logs a message at the debug level.
#[macro_export]
macro_rules! debug {
    // debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Debug), format!($($arg)+)))
}

/// 📰 - Logs a message at the info level.
#[macro_export]
macro_rules! info {
    // info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Info), format!($($arg)+)))
}

/// 🎉 - Logs a message at the success level.
#[macro_export]
macro_rules! success {
    // success!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Success), format!($($arg)+)))
}

/// 🔧 - Logs a message at the trace level.
#[macro_export]
macro_rules! trace {
    // trace!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Trace), format!($($arg)+)))
}

/// 💭 - Logs a message at the help level.
#[macro_export]
macro_rules! help {
    // help!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Help), format!($($arg)+)))
}

/// 🧠 - Logs a message at the custom level.
#[macro_export]
macro_rules! custom {
    // custom!("a {} event", "log")
    ($level: expr, $message: expr) => {
        $crate::log($crate::Flag::String($level), $message)
    };
}

/// 😵 - Logs a message at the Fatal level.
#[macro_export]
macro_rules! fatal {
    // fatal!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Flag::Level($crate::Level::Fatal), format!($($arg)+)))
}

////// ! Test Section ! //////

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Macro output captures.
    fn macros() {
        trace!("Trace message."); // 02:12:22 | 🔧 TRCE | Trace message.
        debug!("Debug message."); // 02:12:22 | 🔮 DBUG | Debug message.
        info!("Info message."); // 02:12:22 | 📰 INFO | Info message.
        success!("Success message."); // 02:12:22 | 🎉 YEEE | Success message.
        warn!("Warn message."); // 02:12:22 | 💡 WARN | Warn message.
        error!("Error message."); // 02:12:22 | 💥 F#CK | Error message.
        custom!("🧭 CSTM".to_string(), format!("Custom message.")); // 20:39:24 | 🧭 CSTM | Custom message.
    }
}

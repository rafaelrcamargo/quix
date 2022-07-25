//! Loggy library
//! This library is used to log messages in a structured and beautiful way.
//!
//! # Usage
//! This lib uses the `println` macro to log messages, exposes some macros to log messages in different levels.
//! ```rust
//! #[macro_use]
//! extern crate loggy;
//!
//! fn main() {
//!    debug!("Starting the CLI."); // 20:39:24 | 🔮 DBUG | Starting the CLI.
//!    warn!("Starting the CLI."); // 20:39:24 | 💡 WARN | Starting the CLI.
//!    error!("Starting the CLI."); // 20:39:24 | 💥 F#CK | Starting the CLI.
//!    trace!("Starting the CLI."); // 20:39:24 | 🔧 TRCE | Starting the CLI.
//!    success!("Starting the CLI."); // 20:39:24 | 🎉 YEEE | Starting the CLI.
//!
//!    // Thats a different macro, here we can define the level of the message as the way we want.
//!    custom!("🧭 CSTM", format!("Starting the {}.", "CLI")); // 20:39:24 | 🧭 CSTM | Starting the CLI.
//! }
//! ```

use colored::{ColoredString, Colorize};
use colored_json::{Color, ColorMode, Styler, ToColoredJson};
use serde_json::Value;
use std::fmt;

/// # Level enum.
/// This enum is used to define the level of the message.
/// The levels are:
/// - `Log`: Used to log messages.
/// - `Debug`: Used to log debug messages.
/// - `Warn`: Used to log warning messages.
/// - `Error`: Used to log error messages.
/// - `Success`: Used to log success messages.
/// - `Trace`: Used to log trace messages.
#[derive(Debug)]
pub enum Level {
    Warn,
    Help,
    Debug,
    Error,
    Trace,
    Success,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Debug => write!(f, "{}", "🔮 DBUG".bright_blue()),
            Level::Warn => write!(f, "{}", "💡 WARN".bright_yellow()),
            Level::Success => write!(f, "{}", "🎉 YEEE".blink().bright_green()),
            Level::Trace => write!(f, "{}", "🔧 TRCE".bright_black()),
            Level::Help => write!(f, "{}", "💭 HELP".normal()),
            Level::Error => write!(
                f,
                "{}",
                format!("💥 {}", "F#CK".strikethrough()).bright_red()
            ),
        }
    }
}

/// The standard logging function.
pub fn log(level: Level, message: fmt::Arguments) {
    // ? Logging date and time.
    let date = chrono::offset::Local::now(); // Get the current date.
    let date = date.format("%H:%M:%S").to_string(); // Format the date.
    let date = date.dimmed();

    // ? Logging message.
    let message = normalize_message(message.to_string(), &level); // Normalize + format the message.

    println!("{} | {} | {}", date, level, message);
}

/// The custom logging function.
pub fn custom_log(level: String, message: String) {
    // ? Logging date and time.
    let date = chrono::offset::Local::now(); // Get the current date.
    let date = date.format("%H:%M:%S").to_string(); // Format the date.
    let date = date.dimmed();

    println!("{} | {} | {}", date, level, message);
}

/// The standard logging function.
pub fn json_log(message: &str) {
    // ? Logging date and time.
    let date = chrono::offset::Local::now(); // Get the current date.
    let date = date.format("%H:%M:%S").to_string(); // Format the date.
    let date = date.dimmed();

    let level = "📃 JSON".normal(); // Set the level to JSON.

    let object: Value = serde_json::from_str(message).unwrap();
    let json = serde_json::to_string_pretty(&object).unwrap(); // Convert to JSON.
    let json = json // Pretty print the JSON.
        .to_colored_json_with_styler(
            ColorMode::default().eval(),
            Styler {
                key: Color::Red.normal(),
                string_value: Color::Green.normal(),
                integer_value: Color::Yellow.normal(),
                float_value: Color::Yellow.normal(),
                object_brackets: Color::Purple.normal(),
                array_brackets: Color::Cyan.normal(),
                ..Default::default()
            },
        )
        .unwrap();

    // ? Final message.
    println!("{} | {} | Data: \n\n{}\n", date, level, json);
}

fn normalize_message(message: String, level: &Level) -> ColoredString {
    match level {
        Level::Debug => message.bright_blue(),
        Level::Warn => message.bright_yellow().underline(),
        Level::Error => message.bright_red().bold(),
        Level::Trace => message.dimmed(),
        Level::Help => message.italic(),
        Level::Success => message.bright_green(),
    }
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```edition2018
/// use log::error;
///
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// ```
#[macro_export]
macro_rules! error {
    // error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Error, format_args!($($arg)+)))
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```edition2018
/// use log::warn;
///
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// ```
#[macro_export]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Warn, format_args!($($arg)+)))
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```edition2018
/// use log::debug;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! debug {
    // debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Debug, format_args!($($arg)+)))
}

/// Logs a message at the success level.
///
/// # Examples
///
/// ```edition2018
/// use log::success;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// success!("New position: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! success {
    // success!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Success, format_args!($($arg)+)))
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```edition2018
/// use log::trace;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! trace {
    // trace!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Trace, format_args!($($arg)+)))
}

/// Logs a message at the help level.
///
/// # Examples
///
/// ```edition2018
/// use log::help;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// help!("Position is: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! help {
    // help!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log($crate::Level::Help, format_args!($($arg)+)))
}

/// Logs a message at the custom level.
///
/// # Examples
///
/// ```edition2018
/// use log::custom;
///
///
/// custom!("🧭 CSTM", format!("Starting the {}.", "CLI"));
/// ```
#[macro_export]
macro_rules! custom {
    // custom!("a {} event", "log")
    ($level: expr, $message: expr) => {
        $crate::custom_log($level, $message)
    };
}

#[macro_export]
macro_rules! stringify {
    // custom!("a {} event", "log")
    ($json: expr) => {
        $crate::json_log($json)
    };
}

use colored::Colorize;
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
    Success,
    Trace,
    Debug,
    Info,
    Help,
    Warn,
    Error,
    Fatal,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Debug => write!(f, "{}", "🔮 DBUG".bright_blue()),
            Level::Info => write!(f, "{}", "📰 INFO".normal()),
            Level::Warn => write!(f, "{}", "💡 WARN".bright_yellow()),
            Level::Success => write!(f, "{}", "🎉 YEEE".blink().bright_green()),
            Level::Trace => write!(f, "{}", "🔧 TRCE".bright_black()),
            Level::Help => write!(f, "{}", "💭 HELP".normal()),
            Level::Error => write!(
                f,
                "{}",
                format!("💥 {}", "F#CK".strikethrough()).bright_red()
            ),
            Level::Fatal => write!(f, "{}", "😵 FATL".on_black()),
        }
    }
}

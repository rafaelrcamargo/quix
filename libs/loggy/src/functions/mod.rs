use std::{fmt::Arguments, process::exit};

use colored::{ColoredString, Colorize};
use colored_json::{Color, ColorMode, ToColoredJson};
use serde_json::Value;

use crate::Level;

/// The standard logging function.
pub fn log(level: Level, message: Arguments) -> () {
    // ? Logging date and time.
    let date = chrono::offset::Local::now(); // Get the current date.
    let date = date.format("%H:%M:%S").to_string(); // Format the date.
    let date = date.dimmed();

    // ? Logging message.
    let message = normalize_message(message.to_string(), &level); // Normalize + format the message.

    println!("{} | {} | {}", date, level, message);

    match level {
        Level::Error => exit(exitcode::UNAVAILABLE),
        _ => (),
    }
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
            colored_json::Styler {
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

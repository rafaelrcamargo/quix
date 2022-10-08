use std::fmt::Arguments;

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
    let message = message.to_string();
    let message = if message.starts_with("\n") {
        normalize_message(format!("\n{}", message), &level)
    } else {
        normalize_message(message, &level)
    };

    println!("{} | {} | {}", date, level, message);

    /* match level {
        Level::Error => exit(exitcode::UNAVAILABLE),
        _ => (),
    } */
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

    // * Check if the message is a valid JSON.
    let json = match serde_json::from_str::<Value>(message) {
        Ok(message) => {
            let json = serde_json::to_string_pretty(&message).unwrap();
            json // Pretty print the JSON.
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
                .unwrap()
        }
        Err(_) => message.to_string(),
    }; // Parse the message as JSON.

    // ? Final message.
    println!("{} | {} | Data: \n\n{}\n", date, level, json);
}

fn normalize_message(message: String, level: &Level) -> ColoredString {
    match level {
        Level::Debug => message.normal(),
        Level::Info => message.normal(),
        Level::Warn => message.bright_yellow().bold(),
        Level::Error => message.bright_red().underline(),
        Level::Trace => message.dimmed(),
        Level::Help => message.italic(),
        Level::Success => message.bright_green(),
    }
}

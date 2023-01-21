use pcre2::bytes::Regex;

use colored::{ColoredString, Colorize};
use colored_json::{Color, ColorMode, ToColoredJson};
use serde_json::Value;

use crate::Level;

/// Parse JSON & pretty print.
fn json_prettifier(message: &str) -> String {
    match serde_json::from_str::<Value>(message) {
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
    } // Parse the message as JSON.
}

/// Syntax parser, extract JSON from messages and parse it.
fn syntax_parser(message: &str) -> String {
    let mut out: String = message.to_string();
    for m in Regex::new(r"{([^{}]|(?R))*}")
        .unwrap()
        .find_iter(message.as_bytes())
    {
        let m = m.unwrap();
        let b = &message.as_bytes()[m.start()..m.end()];
        let json = json_prettifier(std::str::from_utf8(b).unwrap());
        out = out.replace(std::str::from_utf8(b).unwrap(), json.as_str());
    }
    out
}

/// Flag either a Level or a String.
#[derive(Debug, Clone)]
pub enum Flag {
    Level(Level),
    String(String),
}

/// The standard logging function.
pub fn log(level: Flag, message: String) {
    // ? Logging date and time.
    let date = chrono::offset::Local::now(); // Get the current date.
    let date = date.format("%H:%M:%S").to_string(); // Format the date.
    let date = date.dimmed();

    // ? Logging message.
    let message = syntax_parser(message.as_str()); // Parse the message.

    match level {
        Flag::Level(level) => {
            let message = if message.starts_with('\n') {
                normalize_message(format!("\n{}", message), &level)
            } else {
                normalize_message(message, &level)
            };

            println!("{} | {} | {}", date, level, message);

            if let Level::Fatal = level {
                panic!();
            }
        }
        Flag::String(level) => {
            println!("{} | {} | {}", date, level.normal(), message.normal())
        }
    }
}

/// Normalize the message level.
fn normalize_message(message: String, level: &Level) -> ColoredString {
    match level {
        Level::Debug => message.normal(),
        Level::Info => message.normal(),
        Level::Warn => message.bright_yellow().bold(),
        Level::Error => message.bright_red().bold(),
        Level::Trace => message.dimmed(),
        Level::Help => message.italic(),
        Level::Success => message.bright_green(),
        Level::Fatal => message.bold(),
    }
}

//////// ! Test Section ! ////////

#[cfg(test)]
mod tests {
    use super::{json_prettifier, normalize_message, syntax_parser};

    use crate::Level;
    use colored::Styles;

    // Grab the return type.
    fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test] // Test the json_prettifier returned content.
    fn json() {
        let json = "{\"message\":\"Hello, world!\"}";

        let pretty_json = json_prettifier(json);
        println!("JSON: {}", pretty_json); // Send capture.

        // ! Check if the returned content is different from the original.
        assert_ne!(json, pretty_json);
    }

    #[test] // Check the returned type of the syntax_parser.
    fn syntax() {
        let message = "Lorem ipsum dolor sit amet consectetur adipiscing elit:\n{\"foo\":{\"bar\":\"\",\"baz\":0}}\nSunt in culpa qui officia deserunt mollit anim id est laborum.";

        let parsed_message = syntax_parser(message);
        println!("Message: {}", parsed_message); // Send capture.

        // ! Assert the final type of the returned value.
        assert!(type_of(parsed_message).contains("String"));
    }

    #[test] // Affirm the returned flag from normalize_message.
    fn normalize() {
        let help = normalize_message("Help".to_string(), &Level::Help);
        let warn = normalize_message("Warn".to_string(), &Level::Warn);
        let trace = normalize_message("Trace".to_string(), &Level::Trace);

        println!("{} | {} | {}", help, warn, trace); // Send capture.

        // ! Assert the final type of the returned value.
        assert!(help.style().contains(Styles::Italic));
        assert!(warn.style().contains(Styles::Bold));
        assert!(trace.style().contains(Styles::Dimmed));
    }
}

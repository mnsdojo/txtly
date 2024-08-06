// src/format.rs

/// Enum representing various text formatting options for terminal output.
#[derive(Debug, Clone, Copy)]
pub enum Format {
    Bold,
    Underline,
    Italic,
    Blink,
    Reverse,
}

impl Format {
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Format::Bold => "\x1b[1m",      // Bold text
            Format::Underline => "\x1b[4m", // Underlined text
            Format::Italic => "\x1b[3m",    // Italic text
            Format::Blink => "\x1b[5m",     // Blinking text
            Format::Reverse => "\x1b[7m",   // Reversed text colors
        }
    }
}

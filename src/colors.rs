// src/colors.rs

/// Enum representing various colors for text and background in the terminal.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightYellow,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {

    pub(crate) fn fg_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",           // Red text
            Color::Green => "\x1b[32m",         // Green text
            Color::Blue => "\x1b[34m",          // Blue text
            Color::Yellow => "\x1b[33m",        // Yellow text
            Color::Magenta => "\x1b[35m",       // Magenta text
            Color::Cyan => "\x1b[36m",          // Cyan text
            Color::White => "\x1b[37m",         // White text
            Color::Black => "\x1b[30m",         // Black text
            Color::BrightRed => "\x1b[91m",     // Bright Red text
            Color::BrightGreen => "\x1b[92m",   // Bright Green text
            Color::BrightBlue => "\x1b[94m",    // Bright Blue text
            Color::BrightYellow => "\x1b[93m",  // Bright Yellow text
            Color::BrightMagenta => "\x1b[95m", // Bright Magenta text
            Color::BrightCyan => "\x1b[96m",    // Bright Cyan text
            Color::BrightWhite => "\x1b[97m",   // Bright White text
        }
    }
    

    pub(crate) fn bg_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[41m",            // Red background
            Color::Green => "\x1b[42m",          // Green background
            Color::Blue => "\x1b[44m",           // Blue background
            Color::Yellow => "\x1b[43m",         // Yellow background
            Color::Magenta => "\x1b[45m",        // Magenta background
            Color::Cyan => "\x1b[46m",           // Cyan background
            Color::White => "\x1b[47m",          // White background
            Color::Black => "\x1b[40m",          // Black background
            Color::BrightRed => "\x1b[101m",     // Bright Red background
            Color::BrightGreen => "\x1b[102m",   // Bright Green background
            Color::BrightBlue => "\x1b[104m",    // Bright Blue background
            Color::BrightYellow => "\x1b[103m",  // Bright Yellow background
            Color::BrightMagenta => "\x1b[105m", // Bright Magenta background
            Color::BrightCyan => "\x1b[106m",    // Bright Cyan background
            Color::BrightWhite => "\x1b[107m",   // Bright White background
        }
    }
}

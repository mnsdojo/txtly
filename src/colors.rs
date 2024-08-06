// src/colors.rs
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
    // this method is public within the crate
    pub(crate) fn fg_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
            Color::BrightRed => "\x1b[91m",
            Color::BrightGreen => "\x1b[92m",
            Color::BrightBlue => "\x1b[94m",
            Color::BrightYellow => "\x1b[93m",
            Color::BrightMagenta => "\x1b[95m",
            Color::BrightCyan => "\x1b[96m",
            Color::BrightWhite => "\x1b[97m",
        }
    }

    pub(crate) fn bg_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Blue => "\x1b[44m",
            Color::Yellow => "\x1b[43m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
            Color::Black => "\x1b[40m",
            Color::BrightRed => "\x1b[101m",
            Color::BrightGreen => "\x1b[102m",
            Color::BrightBlue => "\x1b[104m",
            Color::BrightYellow => "\x1b[103m",
            Color::BrightMagenta => "\x1b[105m",
            Color::BrightCyan => "\x1b[106m",
            Color::BrightWhite => "\x1b[107m",
        }
    }
}

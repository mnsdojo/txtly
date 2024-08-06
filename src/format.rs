// src/format.rs
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
            Format::Bold => "\x1b[1m",
            Format::Underline => "\x1b[4m",
            Format::Italic => "\x1b[3m",
            Format::Blink => "\x1b[5m",
            Format::Reverse => "\x1b[7m",
        }
    }
}

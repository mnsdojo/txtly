// // src/txtly.rs

// use crate::{Color, Format};
// use std::fmt;

// pub struct Txtly<T: fmt::Display> {
//     content: T,
//     fg_color: Option<Color>,
//     bg_color: Option<Color>,
//     format: Option<Format>,
// }

// impl<T: fmt::Display> Txtly<T> {
//     /// Creates a new `Txtly` instance with the given content.
//     pub fn new(content: T) -> Self {
//         Txtly {
//             content,
//             fg_color: None,
//             bg_color: None,
//             format: None,
//         }
//     }

//     /// Sets the foreground color and formatting for the text.
//     pub fn style(mut self, color: Color, format: Format) -> Self {
//         self.fg_color = Some(color);
//         self.format = Some(format);
//         self
//     }

//     /// Sets the background color for the text.
//     pub fn bg(mut self, color: Color) -> Self {
//         self.bg_color = Some(color);
//         self
//     }
// }

// impl<T: fmt::Display> fmt::Display for Txtly<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let fg_code = self.fg_color.map_or("", |c| c.fg_code());
//         let bg_code = self.bg_color.map_or("", |c| c.bg_code());
//         let format_code = self.format.map_or("", |f| f.code());

//         write!(
//             f,
//             "{}{}{}{}{}", // Formatting sequence
//             fg_code,
//             bg_code,
//             format_code,
//             self.content,
//             "\x1b[0m" // Reset
//         )
//     }
// }

use crate::{Color, Format};
use std::fmt;

pub struct Txtly<T: fmt::Display> {
    content: T,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    format: Option<Format>,
}

impl<T: fmt::Display> Txtly<T> {
    /// Creates a new `Txtly` instance with the given content.
    pub fn new(content: T) -> Self {
        Txtly {
            content,
            fg_color: None,
            bg_color: None,
            format: None,
        }
    }

    /// Sets the foreground color for the text.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Sets the background color for the text.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Sets the format (e.g., bold, underline) for the text.
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
}

impl<T: fmt::Display> fmt::Display for Txtly<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fg_code = self.fg_color.map_or("", |c| c.fg_code());
        let bg_code = self.bg_color.map_or("", |c| c.bg_code());
        let format_code = self.format.map_or("", |f| f.code());

        write!(
            f,
            "{}{}{}{}{}", // Formatting sequence
            fg_code,
            bg_code,
            format_code,
            self.content,
            "\x1b[0m" // Reset
        )
    }
}

// src/lib.rs

mod colors;
mod format;
mod txtly;

pub use colors::Color;
pub use format::Format;
pub use txtly::Txtly;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styled_text() {
        let text = Txtly::new("Hello").style(Color::Red, Format::Bold); // Updated to use the new method name
        assert_eq!(format!("{}", text), "\x1b[31m\x1b[1mHello\x1b[0m");
    }

    #[test]
    fn test_background_color() {
        let text = Txtly::new("Hello").bg(Color::BrightYellow); // Updated to use the new method name
        assert_eq!(format!("{}", text), "\x1b[103mHello\x1b[0m");
    }
}

use txtly::{Color, Format, Txtly};

fn main() {
    // Basic styled text
    let basic_styled_text = Txtly::new("Hello, World!").style(Color::Red, Format::Bold);
    println!("{}", basic_styled_text);

    // Text with background color
    let bg_color_text = Txtly::new("This has a yellow background").bg(Color::BrightYellow);
    println!("{}", bg_color_text);

    // Combining multiple styles
    let combined_styles = Txtly::new("Stylish Text!")
        .style(Color::Green, Format::Bold)
        .bg(Color::Blue);
    println!("{}", combined_styles);

    // Italic text
    let italic_text = Txtly::new("This text is italic").style(Color::Blue, Format::Italic);
    println!("{}", italic_text);

    // Underlined text
    let underlined_text =
        Txtly::new("This text is underlined").style(Color::Red, Format::Underline);
    println!("{}", underlined_text);

    // Chain multiple styles together
    let chain_styles = Txtly::new("Chained Styles")
        .style(Color::BrightYellow, Format::Bold)
        .bg(Color::Green);
    println!("{}", chain_styles);

    // Using default formatting (just text)
    let plain_text = Txtly::new("Just plain text");
    println!("{}", plain_text);
}

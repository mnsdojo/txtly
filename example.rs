use txtly::{Color, Format, Txtly};

fn main() {
    // Basic styled text: Red foreground and bold
    let basic_styled_text = Txtly::new("Hello, World!")
        .fg(Color::Red)
        .format(Format::Bold);
    println!("{}", basic_styled_text);

    // Text with background color: Yellow background
    let bg_color_text = Txtly::new("This has a yellow background").bg(Color::BrightYellow);
    println!("{}", bg_color_text);

    // Combining multiple styles: Green foreground, Bold, Blue background
    let combined_styles = Txtly::new("Stylish Text!")
        .fg(Color::Green)
        .format(Format::Bold)
        .bg(Color::Blue);
    println!("{}", combined_styles);

    // Italic text: Blue foreground and italic
    let italic_text = Txtly::new("This text is italic")
        .fg(Color::Blue)
        .format(Format::Italic);
    println!("{}", italic_text);

    // Underlined text: Red foreground and underlined
    let underlined_text = Txtly::new("This text is underlined")
        .fg(Color::Red)
        .format(Format::Underline);
    println!("{}", underlined_text);

    // Chain multiple styles: Yellow foreground, Bold, Green background
    let chain_styles = Txtly::new("Chained Styles")
        .fg(Color::BrightYellow)
        .format(Format::Bold)
        .bg(Color::Green);
    println!("{}", chain_styles);

    // Using default formatting (just text, no styles)
    let plain_text = Txtly::new("Just plain text");
    println!("{}", plain_text);
}

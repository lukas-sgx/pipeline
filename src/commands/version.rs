pub fn actual() {
    println!(
        "{} v{} by {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        colorized::colorize_this(env!("CARGO_PKG_AUTHORS"), colorized::Colors::CyanFg),
    )
}

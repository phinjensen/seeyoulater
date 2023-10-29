pub enum Color {
    Green,
    Yellow,
    Blue,
    Cyan,
    BoldGreen,
}

pub fn color(string: &str, color: Color) -> String {
    format!(
        "\x1b[{}m{}\x1b[m",
        match color {
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Cyan => "36",
            Color::BoldGreen => "1;32",
        },
        string
    )
}

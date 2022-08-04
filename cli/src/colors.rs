pub enum Color {
    Cyan,
    Green,
    Yellow,
    BoldGreen,
}

pub fn color(string: &str, color: Color) -> String {
    format!(
        "\x1b[{}m{}\x1b[m",
        match color {
            Color::BoldGreen => "1;32",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Cyan => "36",
        },
        string
    )
}

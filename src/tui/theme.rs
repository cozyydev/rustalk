use ratatui::style::Color;

pub struct Theme {
    pub bg: Color,
    pub panel_border: Color,
    pub title: Color,
    pub status: Color,
    pub text: Color,
    pub muted: Color,
    pub system: Color,
    pub error: Color,
    pub self_name: Color,
    pub other_name: Color,
    pub timestamp: Color,
    pub input_border: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub header_accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Rgb(40, 42, 54),
            panel_border: Color::Rgb(98, 114, 164),
            title: Color::Rgb(189, 147, 249),
            status: Color::Rgb(80, 250, 123),
            text: Color::Rgb(248, 248, 242),
            muted: Color::Rgb(98, 114, 164),
            system: Color::Rgb(139, 233, 253),
            error: Color::Rgb(255, 85, 85),
            self_name: Color::Rgb(80, 250, 123),
            other_name: Color::Rgb(255, 184, 108),
            timestamp: Color::Rgb(189, 147, 249),
            input_border: Color::Rgb(255, 121, 198),
            header_bg: Color::Rgb(68, 71, 90),
            header_fg: Color::Rgb(248, 248, 242),
            header_accent: Color::Rgb(189, 147, 249),
        }
    }
}

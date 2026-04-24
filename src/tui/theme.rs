use ratatui::style::Color;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
    pub name_palette: [Color; 6],
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
            self_name: Color::Rgb(0, 209, 171),
            name_palette: [
                Color::Rgb(255, 184, 108), // orange
                Color::Rgb(255, 121, 198), // pink
                Color::Rgb(139, 233, 253), // cyan
                Color::Rgb(189, 147, 249), // purple
                Color::Rgb(241, 250, 140), // yellow
                Color::Rgb(80, 250, 123),  // green
            ],
            timestamp: Color::Rgb(189, 147, 249),
            input_border: Color::Rgb(255, 121, 198),
            header_bg: Color::Rgb(68, 71, 90),
            header_fg: Color::Rgb(248, 248, 242),
            header_accent: Color::Rgb(189, 147, 249),
        }
    }
}

impl Theme {
    pub fn color_for_name(&self, nick: &str, self_nick: &str) -> Color {
        if nick == self_nick {
            return self.self_name;
        }

        let mut hasher = DefaultHasher::new();
        nick.hash(&mut hasher);
        let idx = (hasher.finish() as usize) % self.name_palette.len();
        self.name_palette[idx]
    }
}

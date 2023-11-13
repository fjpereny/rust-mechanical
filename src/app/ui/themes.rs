pub enum Theme {
    Default,
}

pub mod palettes {
    use ratatui::style::Color;

    pub struct Palette {
        pub header_color_1: Color,
        pub row_color_fg_a: Color,
        pub row_color_bg_a: Color,
        pub row_color_fg_b: Color,
        pub row_color_bg_b: Color,
    }

    impl Palette {
        pub fn new() -> Self {
            DEFAULT_PALETTE
        }
    }

    pub const DEFAULT_PALETTE: Palette = Palette {
        header_color_1: Color::LightGreen,
        row_color_fg_a: Color::LightBlue,
        row_color_bg_a: Color::Rgb(30, 30, 30),
        row_color_fg_b: Color::Rgb(200, 200, 200),
        row_color_bg_b: Color::Rgb(10, 10, 10),
    };
}

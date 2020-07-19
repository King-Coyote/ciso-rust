use sfml::graphics::{
    Font,
    Color,
};

#[derive(Clone,)]
pub struct Style {
    pub background_color: Color,
}

impl Style {
    pub fn new(color: Color) -> Self {
        Style {
            background_color: color,
        }
    }

    pub fn new_default() -> Self {
        Style {
            background_color: Color::WHITE,
        }
    }
}
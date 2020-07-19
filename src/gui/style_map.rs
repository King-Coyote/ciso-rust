use std::collections::HashMap;
use crate::gui::{WidgetState, Style,};
use sfml::graphics::Color;

pub struct StyleMap {
    map: HashMap<WidgetState, Style>,
}

impl StyleMap {
    pub fn new() -> Self {
        StyleMap {
            map: [
                (WidgetState::Hovered, Style::new(Color::RED)),
                (WidgetState::Enabled, Style::new(Color::WHITE)),
                (WidgetState::Clicked, Style::new(Color::GREEN))
            ].iter().cloned().collect()
        }
    }

    pub fn get_style(&self, state: &WidgetState) -> Option<&Style> {
        self.map.get(&state)
    }
}
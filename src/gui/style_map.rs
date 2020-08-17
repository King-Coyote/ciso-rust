use std::collections::HashMap;
use crate::gui::{WidgetState, Style,};
use sfml::graphics::Color;
use rlua::{Table,};

pub struct StyleMap {
    map: HashMap<WidgetState, Style>,
}

impl StyleMap {
    pub fn new(style_table: &Table) -> Self {
        let default_style = Style::default();
        let mut style_map = StyleMap {map: HashMap::new()};
        // default style
        style_map.map.insert(WidgetState::Null, Style::default());
        // styles from lua
        style_table.get::<_, Table>("hovered")
            .map(|t| style_map.map.insert(WidgetState::Hovered, Style::new(&t))).ok();
        style_table.get::<_, Table>("enabled")
            .map(|t| style_map.map.insert(WidgetState::Enabled, Style::new(&t))).ok();
        style_table.get::<_, Table>("disabled")
            .map(|t| style_map.map.insert(WidgetState::Disabled, Style::new(&t))).ok();
        style_table.get::<_, Table>("clicked")
            .map(|t| style_map.map.insert(WidgetState::Clicked, Style::new(&t))).ok();
        style_map
    }

    pub fn get_style(&self, state: &WidgetState) -> &Style {
        if let Some(style) = self.map.get(&state) {
            return style;
        }
        return self.map.get(&WidgetState::Null).expect("there must be a default state in the style map.");
    }
}
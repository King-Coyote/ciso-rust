use std::collections::HashMap;
use crate::gui::{WidgetState, Style,};

pub struct StyleMap {
    map: HashMap<WidgetState, Style>,
}

impl StyleMap {
    pub fn new() -> Self {
        StyleMap {
            map: HashMap::new(),
        }
    }

    pub fn get_style(&self, state: WidgetState) -> Option<&Style> {
        self.map.get(&state)
    }
}
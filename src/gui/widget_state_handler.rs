use crate::events::*;
use crate::gui::Style;
use sfml::window::{
    {Event as SFEvent, Key},
    mouse::Button,
};
use sfml::system::{Vector2f,};
use sfml::graphics::{FloatRect,};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum WidgetState {
    Disabled,
    Enabled,
    Hovered,
    Clicked,
    Unclicked,
}

// TODO make a macro for this??
trait BoundsPred: FnOnce(Vector2f) -> bool {}
impl<U> BoundsPred for U where U: FnOnce(Vector2f) -> bool {}

pub struct WidgetStateHandler {
    pub closed: bool,
    pub hidden: bool,
    pub state: WidgetState,
    style_map: HashMap<WidgetState, Style>,
}

impl WidgetStateHandler {
    pub fn new() -> WidgetStateHandler {
        WidgetStateHandler {
            closed: false,
            hidden: false,
            state: WidgetState::Enabled,
            style_map: HashMap::new(),
        }
    }

    // TODO this should return a style, not a bool
    pub fn handle_state (
        &mut self,
        bounds: FloatRect,
        sf_event: &SFEvent,
    ) -> Option<WidgetState> {
        if self.state == WidgetState::Disabled {
            return None;
        }
        match *sf_event {
            SFEvent::MouseButtonPressed {button, x, y} => {
                self.handle_mouse_pressed(button, x, y, bounds)
            },
            SFEvent::MouseButtonReleased {button, x, y} => {
                self.handle_mouse_released(button, x, y, bounds)
            },
            SFEvent::MouseMoved {x, y} => {
                self.handle_mouse_moved(x, y, bounds)
            },
            SFEvent::KeyReleased {code, ..} => {
                self.handle_key_release(code)
            },
            _ => None,
        }
    }

    fn handle_mouse_pressed(&mut self, button: Button, x: i32, y: i32, bounds: FloatRect) -> Option<WidgetState> {
        match &self.state {
            WidgetState::Hovered => {
                self.state = WidgetState::Clicked;
                Some(self.state.clone())
            },
            _ => None
        }
    }

    fn handle_mouse_released(&mut self, button: Button, x: i32, y: i32, bounds: FloatRect) -> Option<WidgetState> {
        match &self.state {
            WidgetState::Clicked => {
                // handle click event in lua here
                self.state = WidgetState::Enabled;
                Some(self.state.clone())
            },
            _ => None
        }
    }

    fn handle_mouse_moved(&mut self, x: i32, y: i32, bounds: FloatRect) -> Option<WidgetState> {
        None
    }

    fn handle_key_release(&mut self, code: Key) -> Option<WidgetState> {
        None
    }

}
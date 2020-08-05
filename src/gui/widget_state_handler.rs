use crate::events::*;
use crate::gui::Style;
use sfml::window::{
    {Event as SFEvent, Key},
    mouse::Button,
};
use sfml::system::{Vector2f,};
use sfml::graphics::{FloatRect,};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug,)]
pub enum WidgetState {
    Disabled,
    Enabled,
    Hovered,
    Clicked,
    Unclicked,
}

pub struct WidgetStateHandler {
    pub closed: bool,
    pub hidden: bool,
    pub state: WidgetState,
}

impl WidgetStateHandler {
    pub fn new() -> WidgetStateHandler {
        WidgetStateHandler {
            closed: false,
            hidden: false,
            state: WidgetState::Enabled,
        }
    }

    pub fn handle_state (
        &mut self,
        bounds: &FloatRect,
        handled: &mut bool,
        sf_event: &SFEvent,
    ) -> Option<WidgetState> {
        if let Some(new_state) = match *sf_event {
            SFEvent::MouseButtonPressed {button, x, y} => {
                self.handle_mouse_pressed(handled, button, x, y, bounds)
            },
            SFEvent::MouseButtonReleased {button, x, y} => {
                self.handle_mouse_released(handled, button, x, y, bounds)
            },
            SFEvent::MouseMoved {x, y} => {
                self.handle_mouse_moved(handled, x, y, bounds)
            },
            SFEvent::KeyReleased {code, ..} => {
                self.handle_key_release(handled, code)
            },
            _ => None,
        } {
            if new_state != self.state {
                self.state = new_state;
                return Some(new_state);
            }
        }
        None
    }

    fn handle_mouse_pressed(&mut self, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Hovered => {
                new_state = Some(WidgetState::Clicked)
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_released(&mut self, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Clicked => {
                new_state = Some(WidgetState::Hovered);
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_moved(&mut self, handled: &mut bool, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Disabled if free_in_bounds(bounds, x, y, handled) => {
                *handled = true;
            },
            WidgetState::Enabled if free_in_bounds(bounds, x, y, handled) => {
                *handled = true;
                new_state = Some(WidgetState::Hovered)
            },
            WidgetState::Hovered => {
                if !free_in_bounds(bounds, x, y, handled) {
                    new_state = Some(WidgetState::Enabled)
                } else {
                    *handled = true;
                }
            },
            WidgetState::Clicked => {
                if !free_in_bounds(bounds, x, y, handled) {
                    new_state = Some(WidgetState::Enabled)
                } else {
                    *handled = true;
                }
            }
            _ => {}
        };
        return new_state;
    }

    fn handle_key_release(&mut self, handled: &mut bool, code: Key) -> Option<WidgetState> {
        None
    }

    fn some_if_new_state(&mut self, new_state: WidgetState) -> Option<WidgetState> {
        if self.state != new_state {
            self.state = new_state;
            return Some(new_state);
        }
        return None;
    }

}

fn free_in_bounds(bounds: &FloatRect, x: i32, y: i32, handled: &bool) -> bool {
    bounds.contains2(x as f32, y as f32) && !handled
}
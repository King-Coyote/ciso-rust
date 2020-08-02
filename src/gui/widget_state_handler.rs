use crate::events::*;
use crate::gui::Style;
use sfml::window::{
    {Event as SFEvent, Key},
    mouse::Button,
};
use sfml::system::{Vector2f,};
use sfml::graphics::{FloatRect,};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum WidgetState {
    Disabled,
    Enabled,
    Hovered,
    Clicked,
    Unclicked,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum WidgetStateOption {
    Handled(WidgetState),
    NotHandled(WidgetState),
    None
}

use WidgetStateOption::*;

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
        bounds: FloatRect,
        handled: &mut bool,
        sf_event: &SFEvent,
    ) -> Option<WidgetState> {
        if *handled {
            return Option::None;
        }
        let widget_option = match *sf_event {
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
            _ => WidgetStateOption::None,
        };
        return match widget_option {
            WidgetStateOption::Handled(new_state) => {
                *handled = true;
                return self.some_if_new_state(new_state);
            },
            WidgetStateOption::NotHandled(new_state) => {
                return self.some_if_new_state(new_state);
            },
            _ => Option::None
        }
    }

    fn handle_mouse_pressed(&mut self, handled: &mut bool, button: Button, x: i32, y: i32, bounds: FloatRect) -> WidgetStateOption {
        let mut new_state = None;
        match &self.state {
            WidgetState::Hovered => {
                new_state = Handled(WidgetState::Clicked)
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_released(&mut self, handled: &mut bool, button: Button, x: i32, y: i32, bounds: FloatRect) -> WidgetStateOption {
        let mut new_state = None;
        match &self.state {
            WidgetState::Clicked => {
                new_state = Handled(WidgetState::Hovered);
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_moved(&mut self, handled: &mut bool, x: i32, y: i32, bounds: FloatRect) -> WidgetStateOption {
        let mut new_state = None;
        match &self.state {
            WidgetState::Disabled => {
                if bounds.contains2(x as f32, y as f32) {
                    new_state = Handled(WidgetState::Disabled)
                }
            },
            WidgetState::Enabled => {
                if bounds.contains2(x as f32, y as f32) {
                    new_state = Handled(WidgetState::Hovered)
                }
            },
            WidgetState::Hovered => {
                if !bounds.contains2(x as f32, y as f32) {
                    new_state = NotHandled(WidgetState::Enabled)
                }
            },
            WidgetState::Clicked => {
                if !bounds.contains2(x as f32, y as f32) {
                    new_state = NotHandled(WidgetState::Enabled)
                }
            }
            _ => {}
        };
        return new_state;
    }

    fn handle_key_release(&mut self, handled: &mut bool, code: Key) -> WidgetStateOption {
        None
    }

    fn some_if_new_state(&mut self, new_state: WidgetState) -> Option<WidgetState> {
        if self.state != new_state {
            self.state = new_state;
            return Some(new_state);
        }
        return Option::None;
    }

}
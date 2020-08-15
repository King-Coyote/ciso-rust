use sfml::window::{
    {Event as SFEvent, Key},
    mouse::Button,
};
use sfml::graphics::{FloatRect,};
use std::collections::HashMap;
use rlua::{
    Function, 
    Result, 
    Table,
    RegistryKey,
    Context,
    ToLuaMulti,
};
use crate::util::*;
use crate::error::Error;

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
    state_table_key: RegistryKey,
    event_map: HashMap<String, RegistryKey>,
}

impl WidgetStateHandler {
    pub fn new(ctx: &Context, r: RegistryKey) -> WidgetStateHandler {
        WidgetStateHandler {
            closed: false,
            hidden: false,
            state_table_key: r,
            state: WidgetState::Enabled,
            event_map: HashMap::new(),
        }
    }

    pub fn handle_state (
        &mut self,
        bounds: &FloatRect,
        handled: &mut bool,
        sf_event: &SFEvent,
        ctx: &Context,
    ) -> Option<WidgetState> {
        if let Some(new_state) = match *sf_event {
            SFEvent::MouseButtonPressed {button, x, y} => {
                self.handle_mouse_pressed(ctx, handled, button, x, y, bounds)
            },
            SFEvent::MouseButtonReleased {button, x, y} => {
                self.handle_mouse_released(ctx, handled, button, x, y, bounds)
            },
            SFEvent::MouseMoved {x, y} => {
                self.handle_mouse_moved(ctx, handled, x, y, bounds)
            },
            SFEvent::KeyReleased {code, ..} => {
                self.handle_key_release(ctx, handled, code)
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

    pub fn set_properties(&mut self, properties: &Table) -> Result<()> {
        
        Ok(())
    }

    fn handle_mouse_pressed(&mut self, ctx: &Context, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Hovered => {
                new_state = Some(WidgetState::Clicked)
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_released(&mut self, ctx: &Context, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Clicked => {
                self.fire_lua_event(ctx, "onClick", (0, x, y));
                new_state = Some(WidgetState::Hovered);
            },
            _ => {}
        }
        new_state
    }

    fn handle_mouse_moved(&mut self, ctx: &Context, handled: &mut bool, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
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

    fn handle_key_release(&mut self, ctx: &Context, handled: &mut bool, code: Key) -> Option<WidgetState> {
        None
    }


    fn fire_lua_event<'lua, A: ToLuaMulti<'lua>>(&self, ctx: &Context<'lua>, name: &str, args: A) -> Result<()> {
        if let Some(key) = self.event_map.get(name) {
            let func: Function = ctx.registry_value(key)?;
            let this: Table = ctx.registry_value(&self.state_table_key)?;
            func.call::<_, ()>((this, args))?;
            return Ok(());
        }
        wrap_error_for_lua(Error::FunctionNotFound(name.to_owned()))
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
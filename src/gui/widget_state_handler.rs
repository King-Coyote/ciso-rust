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
    Value,
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
    props_table_key: RegistryKey,
}

impl WidgetStateHandler {
    pub fn new<'lua>(ctx: &Context<'lua>, props: Table<'lua>) -> Result<WidgetStateHandler> {
        let key = ctx.create_registry_value(props.clone())?;
        let mut widget_state_handler = WidgetStateHandler {
            closed: false,
            hidden: false,
            props_table_key: key,
            state: WidgetState::Enabled,
        };
        widget_state_handler.set_properties(ctx, props)?;
        Ok(widget_state_handler)
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

    pub fn set_properties<'lua>(&mut self, ctx: &Context<'lua>, new_props: Table<'lua>) -> Result<()> {

        // first update from the table
        new_props.get::<_, bool>("closed").map(|b| self.closed = b).ok();
        new_props.get::<_, bool>("hidden").map(|b| self.hidden = b).ok();

        // update this table
        let current_props: Table = ctx.registry_value(&self.props_table_key)?;
        lua_spread_tables(&current_props, new_props)?;

        Ok(())
    }

    fn handle_mouse_pressed(&mut self, ctx: &Context, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Hovered => {
                new_state = Some(WidgetState::Clicked)
            },
            _ => {
                self.fire_lua_event(ctx, "onUnClick", (0, x, y))
                    .unwrap_or_else(|e| println!("Failed to fire onUnClick event: {}", e));
            }
        }
        new_state
    }

    fn handle_mouse_released(&mut self, ctx: &Context, handled: &mut bool, button: Button, x: i32, y: i32, bounds: &FloatRect) -> Option<WidgetState> {
        let mut new_state = None;
        match &self.state {
            WidgetState::Clicked => {
                self.fire_lua_event(ctx, "onClick", (0, x, y))
                    .unwrap_or_else(|e| println!("Failed to fire onClick event: {}", e));
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
        let properties: Table = ctx.registry_value(&self.props_table_key)?;
        if let Ok(func) = properties.get::<_, Table>("event_handlers")?.get::<_, Function>(name) {
            func.call::<_, ()>((properties, args))?;
        }
        Ok(())
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
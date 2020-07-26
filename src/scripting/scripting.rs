use rlua::prelude::*;
use rlua::Result;
use crate::LuaChannel;
use crate::events::Event;

pub struct Scripting {
    pub lua: Lua
}

impl Scripting {
    pub fn new(event_tx: LuaChannel<Event>) -> Self {
        let this = Scripting {
            lua: Lua::new()
        };
        setup_event_channel(event_tx, &this).expect("Failed to set up lua channel");
        this
    }
}

fn setup_event_channel(event_tx: LuaChannel<Event>, scripting: &Scripting) -> Result<()> {
    scripting.lua.context(|ctx| {
        ctx.globals().set("Event", event_tx)?;
        Ok(())
    })?;
    Ok(())
}
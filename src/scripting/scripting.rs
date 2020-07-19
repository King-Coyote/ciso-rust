use rlua::prelude::*;

pub struct Scripting {
    pub lua: Lua
}

impl Scripting {
    pub fn new() -> Self {
        Scripting {
            lua: Lua::new()
        }
    }
}
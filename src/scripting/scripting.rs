use rlua::prelude::*;
use rlua::{Result, Error, Chunk, Context};
use crate::LuaChannel;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use std::env;
use std::io;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use crate::util;

pub struct Scripting {
    pub lua: Lua
}

impl Scripting {
    pub fn new(event_tx: LuaChannel) -> Self {
        let this = Scripting {
            lua: Lua::new()
        };
        setup_event_channel(event_tx, &this).expect("Failed to set up lua channel");
        this
    }

    // pub fn run_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {

    // }
}

pub fn eval_lua_file<'lua, P, R>(ctx: &Context<'lua>, path: P) -> Result<R> 
    where 
        P: AsRef<Path>,
        R: FromLuaMulti<'lua>
{
    let full_path = util::get_asset_path(path);
    match fs::read_to_string(full_path) {
        Ok(contents) => {
            let parsed = contents.parse::<String>().unwrap();
            let val: R = ctx.load(&parsed).eval()?;
            Ok(val)
        },
        Err(err) => return Err(Error::ExternalError(Arc::new(err)))
    }
}

// do eval but don't worry about return value
pub fn exec_lua_file<'lua, P>(ctx: &Context<'lua>, path: P) -> Result<()> 
    where 
        P: AsRef<Path>,
{
    eval_lua_file::<_, ()>(ctx, path)?;
    Ok(())
}

fn setup_event_channel(event_tx: LuaChannel, scripting: &Scripting) -> Result<()> {
    scripting.lua.context(|ctx| {
        ctx.globals().set("EventChannel", event_tx)?;
        Ok(())
    })?;
    Ok(())
}
use crate::events::*;
use rlua::{
    UserData, 
    UserDataMethods, 
    Result, 
    Table,
};
use rlua::Error;
use crossbeam_channel::Sender;
use std::sync::Arc;

#[derive(Clone,)]
pub struct LuaChannel {
    sender: Sender<Event>,
}

impl LuaChannel {
    pub fn new(sender: Sender<Event>) -> Self {
        LuaChannel {
            sender: sender
        }
    }

    pub fn send(&self, message: Event) -> Result<()> {
        match self.sender.try_send(message) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ExternalError(Arc::new(err)))
        }
    }
}

impl UserData for LuaChannel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("send", |_, channel, table: Table| {
            channel.send(event_from_lua(table)?)?;
            Ok(())
        });
    }
}
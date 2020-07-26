use crate::events::Event;
use rlua::{UserData, UserDataMethods};
use crossbeam_channel::Sender;

#[derive(Clone,)]
pub struct LuaChannel<T> {
    sender: Sender<T>,
}

impl<T> LuaChannel<T> {
    pub fn new(sender: Sender<T>) -> Self {
        LuaChannel {
            sender: sender
        }
    }
}

impl<T> UserData for LuaChannel<T> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("send", |_, channel, ()| {
            Ok(())
        });
    }
}
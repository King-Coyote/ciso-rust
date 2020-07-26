use sfml::window::Event as SFEvent;
use std::hash::{Hash, Hasher};
use crate::gui::Widget;
use rlua::Table;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EventType {
    Input,
    CreateGui,
}

#[derive(Debug, Clone)]
pub enum EventData {
    SFMLInput(SFEvent),
    Filename(String),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub t: EventType,
    pub data: EventData
}

impl Event {
    pub fn new(t: EventType, data: EventData) -> Event {
        Event {
            t: t,
            data: data
        }
    }
    
    pub fn from_lua() -> Self {
        
    }
}
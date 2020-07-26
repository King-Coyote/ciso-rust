use sfml::window::Event as SFEvent;
use std::hash::{Hash, Hasher};
use crate::gui::Widget;
use rlua::{Table, Result};

pub enum Event {
    Input(SFEvent),
    CreateGui(String),
}

// #[derive(Debug, Clone)]
// pub struct Event {
//     pub t: EventType,
//     pub data: EventData
// }

// impl Event {
//     pub fn new(t: EventType, data: EventData) -> Event {
//         Event {
//             t: t,
//             data: data
//         }
//     }
    
//     // pub fn from_lua() -> Self {

//     // }
// }
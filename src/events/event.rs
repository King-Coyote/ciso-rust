use sfml::window::Event as SFEvent;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex,};
use crate::gui::Widget;
use rlua::{Table, Result, Error};

pub enum Event {
    Input(SFEvent),
    CreateGui(u32), // an id that counts up
}

pub fn event_from_lua(t: Table) -> Result<Event> {
    let durr = t.contains_key("type")?;
    let event_type: String = t.get("type")?;
    match &event_type[..] {
        "CREATE_WIDGET" => {
            let id: u32 = t.get("id")?;
            Ok(Event::CreateGui(id))
        },
        _ => {
            Err(Error::FromLuaConversionError{
                from: "Table",
                to: "Ciso Event",
                message: Some("Could not convert table to rust event.".to_owned())
            })
        }
    }
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
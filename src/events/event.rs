use sfml::window::Event as SFEvent;
use rlua::{Table, Result, Error};

pub enum Event {
    Input(SFEvent),
    CreateGui(u32), // an id that counts up
    WidgetChanged(u32), 
}

pub fn event_from_lua(t: Table) -> Result<Event> {
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
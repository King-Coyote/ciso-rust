use sfml::window::Event as SFEvent;
use rlua::{Table, Result, Error, RegistryKey, Context};

pub enum Event {
    Input(SFEvent),
    CreateGui(RegistryKey),
    WidgetChanged(u32, RegistryKey), //the two values are: table changed, and the new properties for it
}

pub fn event_from_lua<'lua>(ctx: &Context<'lua>, t: Table<'lua>) -> Result<Event> {
    let event_type: String = t.get("type")?;
    let key = ctx.create_registry_value(t)?;
    match &event_type[..] {
        "CREATE_WIDGET" => {
            Ok(Event::CreateGui(key))
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
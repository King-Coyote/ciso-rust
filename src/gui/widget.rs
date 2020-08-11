use crate::{
    rendering::Renderer,
    util::*,
    gui::{
        Panel,
    },
};
use sfml::window::Event as SFEvent;
use rlua::{Table, Result, Error};

pub trait Widget
{
    fn draw(&self, dt: f32, renderer: &mut Renderer);
    fn update(&self, dt: f32);
    fn handle_input(&mut self, handled: &mut bool, sf_event: &SFEvent);
    fn is_closed(&self) -> bool;
    fn close(&mut self);
}

pub fn build_widget(t: Table, id: u32) -> Result<Box<dyn Widget>> {
    let widget_type: String = t.get("type")?;
    match &widget_type[..] {
        "PANEL" => {
            build_panel(t, id)
        },
        _ => Err(Error::FromLuaConversionError{
            from: "Table",
            to: "Rust Widget",
            message: Some("Failed trying to construct a rust widget.".to_owned())
        })
    }
}

fn build_panel(t: Table, id: u32) -> Result<Box<dyn Widget>> {
    let size = table_to_pair(t.get("size")?)?;
    let position = table_to_pair(t.get("position")?)?;
    Ok(Box::new(Panel::new(size, position, id)))
}

// pub struct WidgetBuilder;

// impl WidgetBuilder {
//     pub fn start() -> Self {

//     }

//     pub fn 
// }
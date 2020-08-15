use crate::{
    rendering::Renderer,
    util::*,
    gui::{
        Panel,
    },
};
use sfml::window::Event as SFEvent;
use rlua::{Table, Result, Error, Context, RegistryKey};

pub trait Widget
{
    fn draw(&self, dt: f32, renderer: &mut Renderer);
    fn update(&self, dt: f32);
    fn handle_input(&mut self, ctx: &Context, handled: &mut bool, sf_event: &SFEvent);
    fn widget_changed<'lua>(&mut self, ctx: &Context<'lua>, id: u32, new_props: &Table<'lua>) -> Result<()>;
    fn is_closed(&self) -> bool;
    fn close(&mut self);
}

pub fn build_widget(ctx: &Context, key: RegistryKey) -> Result<Box<dyn Widget>> {
    let widget_table: Table = ctx.registry_value(&key)?;
    let widget_type: String = widget_table.get("type")?;
    match &widget_type[..] {
        "PANEL" => {
            build_panel(ctx, widget_table)
        },
        _ => Err(Error::FromLuaConversionError{
            from: "Table",
            to: "Rust Widget",
            message: Some("Failed trying to construct a rust widget.".to_owned())
        })
    }
}

fn build_panel<'lua>(ctx: &Context<'lua>, t: Table<'lua>) -> Result<Box<dyn Widget>> {
    Ok(Box::new(Panel::new(ctx, t)?))
}

// pub struct WidgetBuilder;

// impl WidgetBuilder {
//     pub fn start() -> Self {

//     }

//     pub fn 
// }
use crate::{
    rendering::Renderer,
    util::*,
    gui::{
        Panel,
    },
};
use sfml::{
    window::{Event as SFEvent,},
    system::{Vector2f,},
    graphics::{Transform,},
};
use rlua::{Table, Result, Error, Context, RegistryKey};
use std::sync::atomic::{AtomicU32, Ordering};

static mut ID: AtomicU32 = AtomicU32::new(0);

pub trait Widget
{
    fn draw(&self, dt: f32, renderer: &mut Renderer);
    fn update(&self, dt: f32);
    fn handle_input(&mut self, ctx: &Context, handled: &mut bool, sf_event: &SFEvent);
    fn widget_changed<'lua>(&mut self, ctx: &Context<'lua>, id: u32, new_props: &Table<'lua>) -> Result<()>;
    // update_position is distinct from above so that children can be updated, and also for dragging.
    fn translate(&mut self, delta: (f32, f32));
    fn is_closed(&self) -> bool;
}

pub fn build_widget<'lua>(ctx: &Context<'lua>, widget_table: Table<'lua>) -> Result<Box<dyn Widget>> {
    let widget_type: String = widget_table.get("type")?;
    // this isn't unsafe, shut up rust
    unsafe {widget_table.set("id", ID.fetch_add(1, Ordering::Relaxed))?;}
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
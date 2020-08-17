use crate::{
    rendering::Renderer,
    util::*,
    gui::{
        RectangularPanel,
        Style,
    },
};
use sfml::{
    window::{Event as SFEvent,},
    system::{Vector2f,},
    graphics::{Transform, FloatRect,},
};
use rlua::{Table, Result, Error, Context, RegistryKey};
use std::sync::atomic::{AtomicU32, Ordering};


pub trait Widget
{
    fn draw(&self, dt: f32, renderer: &mut Renderer);
    fn update(&self, dt: f32);
    fn get_bounds(&self) -> FloatRect;
    fn translate(&mut self, delta: (f32, f32));
    fn get_position(&self) -> Vector2f;
    fn update_style(&mut self, style: &Style);
    fn set_properties(&mut self, ctx: &Context, new_props: &Table);
}

pub fn build_widget<'lua>(ctx: &Context<'lua>, widget_table: &Table<'lua>) -> Result<Box<dyn Widget>> {
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

fn build_panel<'lua>(ctx: &Context<'lua>, t: &Table<'lua>) -> Result<Box<dyn Widget>> {
    let panel_shape: String = t.get("shape").unwrap_or("RECTANGLE".to_owned());
    match &panel_shape[..] {
        "CIRCLE" => Ok(Box::new(RectangularPanel::new()?)),
        "SHAPE" => Ok(Box::new(RectangularPanel::new()?)),
        _ => Ok(Box::new(RectangularPanel::new()?))
    }
    
}
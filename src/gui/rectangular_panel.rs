use crate::gui::*;
use crate::rendering::*;
use crate::util;
use sfml::system::Vector2f;
use sfml::graphics::{RectangleShape, FloatRect, Transformable, Shape,};
use rlua::{Context, Table, Result};

// a blank rectangular widget
pub struct RectangularPanel<'s> {
    shape: RectangleShape<'s>,
}

impl<'s> RectangularPanel<'s> {
    pub fn new() -> Result<Self> {
        let panel = RectangularPanel {
            shape: RectangleShape::new(),
        };
        Ok(panel)
    }
}

impl<'s> Widget for RectangularPanel<'s> {
    fn draw(&self, dt: f32, renderer: &mut Renderer) {
        renderer.draw_shape(&self.shape);
    }

    fn update(&self, dt: f32) {

    }

    fn get_bounds(&self) -> FloatRect {
        self.shape.global_bounds()
    }

    fn translate(&mut self, delta: (f32, f32)) {
        self.shape.move_(delta);
    }

    fn get_position(&self) -> Vector2f {
        self.shape.position()
    }

    fn update_style(&mut self, style: &Style) {
        self.shape.set_fill_color(style.background_color);
    }

    fn set_properties(&mut self, ctx: &Context, new_props: &Table) {
        util::lua_get_pair(new_props, "size").map(|v| self.shape.set_size(v)).ok();
    }

}
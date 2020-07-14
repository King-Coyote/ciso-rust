use crate::gui::{
    Widget, 
    WidgetStateHandler,
    WidgetState,
    Style,
    StyleMap,
};
use sfml::graphics::{
    RenderWindow, 
    RectangleShape, 
    RenderTarget,
    Transformable, 
    Color,
    Shape,
};
use crate::events::*;
use crate::rendering::*;
use std::marker::PhantomData;

use sfml::system::{Vector2f,};

pub struct Panel<'s, T: Renderer> {
    shape: RectangleShape<'s>,
    state: WidgetStateHandler,
    styles: StyleMap,
    widget_phantom: PhantomData<T>,
}

impl<'s, T: Renderer> Panel<'s, T> {
    pub fn new<S: Into<Vector2f>>(size: S, pos: S) -> Panel<'s, T> {
        let mut panel_shape = RectangleShape::new();
        panel_shape.set_size(size);
        panel_shape.set_position(pos);
        panel_shape.set_fill_color(Color::WHITE);
        Panel::<T> {
            shape: panel_shape,
            state: WidgetStateHandler::new(),
            styles: StyleMap::new(),
            widget_phantom: PhantomData
        }
    }

    fn update_state(&mut self, new_state: WidgetState) {
        if let Some(style) = self.styles.get_style(new_state) {
            self.shape.set_fill_color(style.background_color);
        }
    }
}

impl<'s, T: Renderer> Widget for Panel<'s, T> 
{
    type R = T;
    fn draw(&self, dt: f32, renderer: &mut T) {
        renderer.draw_shape(&self.shape);
    }

    fn update(&self, dt: f32) {

    }

    fn handle_event(&mut self, event: &mut Event) {
        if let Some(new_state) = match event.data {
            EventData::SFMLInput(data) => {
                self.state.handle_state(
                    self.shape.global_bounds(),
                    &data,
                )
            },
            _ => None
        } {
            self.update_state(new_state);
        }
    }

    fn is_closed(&self) -> bool {
        self.state.closed
    }

    fn close(&mut self) {
        self.state.closed = true;
    }

}
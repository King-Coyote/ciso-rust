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
use sfml::window::Event as SFEvent;

use sfml::system::{Vector2f,};

pub struct Panel<'s, T: Renderer> {
    shape: RectangleShape<'s>,
    state: WidgetStateHandler,
    styles: StyleMap,
    children: Vec<Box<dyn Widget<R = T>>>,
    widget_phantom: PhantomData<T>,
    id: String,
}

impl<'s, T: Renderer + 'static> Panel<'s, T> {
    pub fn new<S: Into<Vector2f>>(size: S, pos: S, id: &str) -> Panel<'s, T> {
        let mut panel_shape = RectangleShape::new();
        panel_shape.set_size(size);
        panel_shape.set_position(pos);
        panel_shape.set_fill_color(Color::WHITE);
        let panel = Panel::<T> {
            shape: panel_shape,
            state: WidgetStateHandler::new(),
            styles: StyleMap::new(),
            children: vec![],
            widget_phantom: PhantomData,
            id: id.to_owned()
        };
        panel
    }

    fn update_state(&mut self, new_state: WidgetState) {
        if let Some(style) = self.styles.get_style(&new_state) {
            self.shape.set_fill_color(style.background_color);
        }
    }

    // probably delete this later dude
    pub fn add_child(&mut self, panel: Box<Panel<'static, T>>) {
        self.children.push(panel);
    }
}

impl<'s, T: Renderer + 'static> Widget for Panel<'s, T>
{
    type R = T;
    fn draw(&self, dt: f32, renderer: &mut T) {
        renderer.draw_shape(&self.shape);
        for child in self.children.iter() {
            child.draw(dt, renderer);
        }
    }

    fn update(&self, dt: f32) {

    }

    fn handle_input(&mut self, handled: &mut bool, sf_event: &SFEvent) {
        for child in self.children.iter_mut() {
            child.handle_input(handled, sf_event);
        }
        if let Some(new_state) = self.state.handle_state(
            &self.shape.global_bounds(),
            handled,
            sf_event
        ) {
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
use crate::gui::{
    Widget, 
    WidgetStateHandler,
    WidgetState,
    StyleMap,
};
use sfml::graphics::{
    RectangleShape, 
    Transformable, 
    Color,
    Shape,
};
use crate::rendering::*;
use sfml::window::Event as SFEvent;

use sfml::system::{Vector2f,};

pub struct Panel<'s> {
    shape: RectangleShape<'s>,
    state: WidgetStateHandler,
    styles: StyleMap,
    children: Vec<Box<dyn Widget>>,
    id: u32
}

impl<'s> Panel<'s> {
    /// Create a new panel
    /// # Arguments
    /// 
    /// * 'size' 
    /// * 'pos'
    /// * 'id'
    pub fn new<S: Into<Vector2f>>(size: S, pos: S, id: u32) -> Panel<'s> {
        let mut panel_shape = RectangleShape::new();
        panel_shape.set_size(size);
        panel_shape.set_position(pos);
        panel_shape.set_fill_color(Color::WHITE);
        let panel = Panel {
            shape: panel_shape,
            state: WidgetStateHandler::new(),
            styles: StyleMap::new(),
            children: vec![],
            id: id
        };
        panel
    }

    fn update_state(&mut self, new_state: WidgetState) {
        if let Some(style) = self.styles.get_style(&new_state) {
            self.shape.set_fill_color(style.background_color);
        }
    }

    // probably delete this later dude
    pub fn add_child(&mut self, panel: Box<Panel<'static>>) {
        self.children.push(panel);
    }
}

impl<'s> Widget for Panel<'s>
{
    fn draw(&self, dt: f32, renderer: &mut Renderer) {
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
use crate::rendering::Renderer;
use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
    Shape, Transformable, Text, Sprite,
};
use sfml::window::{Event, Key, Style, VideoMode};


pub struct SimpleRenderer {
}

impl SimpleRenderer {
    pub fn new() -> Self {SimpleRenderer{}}
}

impl Renderer for SimpleRenderer {
    // call this when drawing begins
    fn begin(&self, window: &mut RenderWindow) {
        window.clear(Color::BLACK);
    }

    fn draw_shape<'a>(&self, window: &mut RenderWindow, drawable: &impl Shape<'a>) {
        window.draw(drawable);
    }

    fn draw_sprite(&self, window: &mut RenderWindow, drawable: &Sprite) {
        window.draw(drawable);
    }

    fn draw_text(&self, window: &mut RenderWindow, drawable: &Text) {
        window.draw(drawable);
    }

    // call this when drawing should end
    fn end(&self, window: &mut RenderWindow) {
        window.display();
    }
}
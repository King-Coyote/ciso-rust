use sfml::graphics::{Shape, Text, Sprite, RenderWindow};

pub trait Renderer {
    // call this when drawing begins
    fn begin(&self, window: &mut RenderWindow);
    fn draw_shape<'a>(&self, window: &mut RenderWindow, drawable: &impl Shape<'a>);
    fn draw_sprite(&self, window: &mut RenderWindow, drawable: &Sprite);
    fn draw_text(&self, window: &mut RenderWindow, drawable: &Text);
    // call this when drawing should end
    fn end(&self, window: &mut RenderWindow);
}